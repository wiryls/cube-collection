namespace entity {
/////////////////////////////////////////////////////////////////////////////

import IWorld = core.IWorld;
import ICube = core.ICube;
import IVec2 = core.IVec2;
import Vec2 = core.Vec2;
import Type = core.Cube.Type;

/////////////////////////////////////////////////////////////////////////////

export class World extends egret.DisplayObjectContainer implements IWorld
{
    private seed_: core.Seed;
    public cube: Array<ICube>;
    public dest: Array<IVec2>;

    constructor()
    {
        super();
    }

    command(code: number)
    {
        switch (code) {
        case input.Controller.Type.MOVE_IDLE:
        case input.Controller.Type.MOVE_LEFT:
        case input.Controller.Type.MOVE_DOWN:
        case input.Controller.Type.MOVE_UP:
        case input.Controller.Type.MOVE_RIGHT: {
            this.cube
                .filter (c => c.type === Type.Blue)
                .forEach(c => c.action = <core.Cube.Action><any>code)
                ;
            break;
        }
        default:
            break;
        }
    }

    next(): void
    {
        core.Transform.link(this.cube, this.size.width, this.size.height);
        core.Transform.move(this.cube, this.size.width, this.size.height);
        for (const c of this.cube)
            c.commit();

        this.cube = this.cube.filter(c => c.live);
    }

    build(): void
    {
        if (this.seed === undefined)
            return;

        this.removeChildren();
        
        const creator = new CubeFactory(this, this);
        this.cube = this.seed.cube.map(c => creator.create(c));
        this.dest = this.seed.dest.map(v => new Vec2(v));
    }

    get size(): { readonly width: number; readonly height: number; }
    {
        if (this.seed !== undefined)
            return this.seed.size;
        else
            return { width: 0, height: 0 };
    }

    get seed(): core.Seed
    {
        return this.seed_;
    }

    set seed(value: core.Seed)
    {
        this.seed_ = value;
        this.build();
    }
}

/////////////////////////////////////////////////////////////////////////////
}