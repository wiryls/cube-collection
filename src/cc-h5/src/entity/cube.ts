namespace entity {
/////////////////////////////////////////////////////////////////////////////

import ICubeFactory = core.ICubeFactory;
import IBehavior = core.IBehavior;
import Behavior = core.Behavior;
import Action = core.Cube.Action;
import Status = core.Cube.Status;
import IWorld = core.IWorld;
import ICube = core.ICube;
import IUnit = core.IUnit;
import IVec2 = core.IVec2;
import Seed = core.Seed;
import Type = core.Cube.Type;

/////////////////////////////////////////////////////////////////////////////

export class Cube extends egret.DisplayObjectContainer implements ICube
{
    private type_: Type;
    private world_: IWorld;
    private entity_: Array<IUnit>;
    private action_: IBehavior;
    private status_: Status;
    private modify_: boolean;

    constructor(owner: IWorld, type: Type, behavior: IBehavior)
    {
        super();

        this.type_ = type;
        this.world_ = owner;
        this.entity_ = new Array<IUnit>();
        this.action_ = behavior;
        this.status_ = Status.Free;
        this.modify_ = false;
    }

    public get type(): Type
    {
        return this.type_;
    }

    public get live(): boolean
    {
        return this.entity_.length > 0;
    }

    public get world(): IWorld
    {
        return this.world_;
    }

    public get moving(): boolean
    {
        return !this.action_.done;
    }

    public get active(): boolean
    {
        return Type.active(this.type);
    }

    public get entity(): Array<IUnit>
    {
        return this.entity_;
    }

    public get behavior(): IBehavior
    {
        return this.action_;
    }

    public get action(): Action
    {
        return this.action_.action;
    }

    public set action(value: Action)
    {
        this.action_.action = value;
    }

    public get status(): Status
    {
        return this.status_;
    }

    public set status(value: Status)
    {
        this.status_ = value;
    }
    
    public absorbable(that: ICube): boolean
    {
        return Type.absorbable(this.type, that.type);
    }

    public absorb(others: Array<ICube>): void
    {
        this.modify_ = true;
        for (const o of others.filter(o => o !== this)) {
            this.entity_.push(...o.entity);
            o.entity.length = 0;
            o.status = Status.Stop;
        }
        this.action_ = Behavior.create(others.concat(this).map(o => o.behavior));
    }

    public commit(): void
    {
        if (!this.live)
            return;

        if (this.moving) {
            for (const e of this.entity)
                e.change(this.action, this.status);

            this.status = Status.Free;
            this.behavior.next();
        }

        if (this.modify_) {
            for (const e of this.entity)
                e.attach(this);

            this.modify_ = false;
        }

        for (const e of this.entity)
            e.commit();
    }
}

/////////////////////////////////////////////////////////////////////////////

class Unit implements IUnit
{
    private modify: boolean = false;
    private action: Action|undefined = undefined;
    private status: Status|undefined = undefined;
    private shape: egret.Shape = new egret.Shape();

    private gridx: number;
    private gridy: number;

    constructor(
        x: number,
        y: number,
        public owner: ICube,
        private readonly stage: egret.DisplayObjectContainer)
    {
        stage.addChild(this.shape);
        owner.entity.push(this);

        this.x = x;
        this.y = y;
        this.color = Unit.toColor(this.owner.type);
    }

    attach(target: ICube): void
    {
        this.modify = true;
        this.owner = target;
    }

    change(action: Action, status: Status): void
    {
        this.action = action;
        this.status = status;
    }

    commit(): void
    {
        if (this.modify === true) {
            this.color = Unit.toColor(this.owner.type);
            this.modify = false;
        }

        if (this.action !== undefined && this.status !== undefined) {
            if (this.action !== Action.Idle && this.status !== Status.Stop) {
                const next = Action.toVec2(this.action).plus(this);
                this.x = next.x;
                this.y = next.y;
            }

            this.action = undefined;
            this.status = undefined;
        }
    }

    get x(): number
    {
        return this.gridx;
    }

    get y(): number
    {
        return this.gridy;
    }

    set x(value: number)
    {
        this.gridx = value;

        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;

        const sid = Math.min(wid / col, hgt / row);
        const tlx = (wid - col * sid) / 2;
        
        const x = tlx + this.x * sid;
        egret.Tween
            .get(this.shape)
            .to({x: x}, 250)
            ;
    }

    set y(value: number)
    {
        this.gridy = value;

        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;

        const sid = Math.min(wid / col, hgt / row);
        const tly = (hgt - row * sid) / 2;

        const y = tly + this.y * sid;
        egret.Tween
            .get(this.shape)
            .to({y: y}, 250)
            ;
    }

    private set color(value: number)
    {
        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;
        const sid = Math.min(wid / col, hgt / row);

        this.shape.graphics.clear();
        this.shape.graphics.beginFill(value);
        this.shape.graphics.drawRect(0, 0, sid + 1, sid + 1);
        this.shape.graphics.endFill();
    }

    private get color(): number
    {
        return Unit.toColor(this.owner.type);
    }

    private static toColor(type: Type): number
    {
        switch(type) {
        default:
        case Type.White: return 0xEAE9E8;
        case Type.Red  : return 0xF33048;
        case Type.Blue : return 0x75C6FF;
        case Type.Green: return 0x75FF81;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

export class CubeFactory implements ICubeFactory
{
    constructor(
        private readonly world: IWorld,
        private readonly stage: egret.DisplayObjectContainer)
    {}

    create(src: Seed.Cube): ICube
    {
        const type = Seed.Cube.toType(src.type);
        const move
            = (src.move === undefined)
            ? (Behavior.create())
            : (Behavior.create(src.move.loop, Seed.Cube.toActions(src.move.path)))
            ;
        const cube = new Cube(this.world, type, move);
        const body = src.body.map(v => new Unit(v[0], v[1], cube, this.stage));

        return cube;
    }
}

/////////////////////////////////////////////////////////////////////////////
}