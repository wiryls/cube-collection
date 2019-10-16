namespace entity {
/////////////////////////////////////////////////////////////////////////////

import IWorld = logic.IWorld;
import ICube = logic.ICube;
import IVec2 = logic.IVec2;
import Vec2 = logic.Vec2;
import Type = logic.Cube.Type;

/////////////////////////////////////////////////////////////////////////////

const Track = utils.Track;
const Musician = utils.Musician.instance;

/////////////////////////////////////////////////////////////////////////////

export class World extends egret.DisplayObjectContainer implements IWorld
{
    private floor: egret.Shape;
    private layer: Array<egret.DisplayObjectContainer>;
    private seed: logic.Seed;

    public cube = new Array<Cube>();
    public dest = new Array<Dest>();

    constructor()
    {
        super();
        this.addEventListener(egret.Event.ADDED_TO_STAGE, this.onAddToStage, this);
        this.floor = new egret.Shape();
        this.layer = Array.of(
            new egret.DisplayObjectContainer(),
            new egret.DisplayObjectContainer(),
            new egret.DisplayObjectContainer(),
            new egret.DisplayObjectContainer()
        );
    }

    private onAddToStage(): void
    {
        for (const layer of this.layer)
            this.addChildAt(layer, 0);

        this.sort();
        this.addChildAt(this.floor, 0);
    }

    public command(code: number): void
    {
        switch (code) {
        case input.Controller.Type.MOVE_LEFT:
        case input.Controller.Type.MOVE_DOWN:
        case input.Controller.Type.MOVE_UP:
        case input.Controller.Type.MOVE_RIGHT:
        case input.Controller.Type.MOVE_IDLE: {
            this.cube
                .filter (c => c.type === Type.Blue)
                .forEach(c => c.action = <logic.Cube.Action><any>code)
                ;
            break;
        }
        default:
            break;
        }
    }

    public next(): void
    {
        logic.Transform.link(this.cube, this.size.width, this.size.height);
        logic.Transform.move(this.cube, this.size.width, this.size.height);
        for (const c of this.cube)
            c.commit();

        this.sort();
        this.cube = this.cube.filter(c => c.live);
    }

    public status(): Array<number>
    {
        return this.dest
            .map(v => this.cube.some(c => c.entity.some(o => o.x === v.x && o.y === v.y)))
            .map(b => b ? 1 : 0)
            ;
    }

    public build(seed?: logic.Seed): void
    {
        if (seed !== undefined)
            this.seed = seed;
        if (this.seed === undefined)
            return;

        this.background();
        for (const layer of this.layer)
            layer.removeChildren();

        const creator = new CubeFactory(this, this.layer);
        this.dest = this.seed.dest.map(v => creator.create(v));
        this.cube = this.seed.cube.map(c => creator.create(c));

        logic.Transform.link(this.cube, this.size.width, this.size.height);
        this.sort();
    }

    public get size(): { readonly width: number; readonly height: number; }
    {
        if (this.seed !== undefined)
            return this.seed.size;
        else
            return { width: 0, height: 0 };
    }

    private sort(): void
    {
        for (const layer of this.layer) {
            layer.$children.sort((l, r) =>
                (l.x + l.y < r.x + r.y) ? -1 :
                (l.x + l.y > r.x + r.y) ? +1 : 0
            );
        }
    }

    private background(): void {
        const {width : col, height : row} = this.size;
        const wid = this.stage.stageWidth;
        const hgt = this.stage.stageHeight;

        const sid = Math.min(wid / col, hgt / row);
        const tlx = (wid - col * sid) / 2;
        const tly = (hgt - row * sid) / 2;

        const floor = this.floor;

        floor.graphics.clear();
        floor.graphics.beginFill(0x1F1F1F);
        floor.graphics.drawRect(0, 0, col * sid, row * sid);
        floor.graphics.endFill();

        floor.x = tlx;
        floor.y = tly;
    }
}

/////////////////////////////////////////////////////////////////////////////
}