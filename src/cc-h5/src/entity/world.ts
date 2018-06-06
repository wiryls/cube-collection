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
    private layer: Array<egret.DisplayObjectContainer>;
    private seed: logic.Seed;

    public cube = new Array<Cube>();
    public dest = new Array<Dest>();

    constructor()
    {
        super();
        this.addEventListener(egret.Event.ADDED_TO_STAGE, this.onAddToStage, this);
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

    next(): void
    {
        logic.Transform.link(this.cube, this.size.width, this.size.height);
        logic.Transform.move(this.cube, this.size.width, this.size.height);
        for (const c of this.cube)
            c.commit();

        this.sort();
        this.cube = this.cube.filter(c => c.live);
    }

    build(): void
    {
        if (this.seed === undefined)
            return;
        this.removeChildren();
        
        const creator = new CubeFactory(this, this);
        this.dest = this.seed.dest.map(v => creator.create(v));
        this.cube = this.seed.cube.map(c => creator.create(c));
    }

    get size(): { readonly width: number; readonly height: number; }
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
}

/////////////////////////////////////////////////////////////////////////////
}