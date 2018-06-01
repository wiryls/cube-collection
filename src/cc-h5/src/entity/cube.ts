namespace entity {
/////////////////////////////////////////////////////////////////////////////

import ICubeFactory = logic.ICubeFactory;
import IBehavior = logic.IBehavior;
import Behavior = logic.Behavior;
import Action = logic.Cube.Action;
import Status = logic.Cube.Status;
import IWorld = logic.IWorld;
import ICube = logic.ICube;
import IUnit = logic.IUnit;
import IVec2 = logic.IVec2;
import Seed = logic.Seed;
import Type = logic.Cube.Type;

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
        this.action_ = Behavior.create(others
            .filter(o => o.absorbable(this))
            .concat(this)
            .map   (o => o.behavior)
        );
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
    private moving: boolean = false;

    private action: Action|undefined = undefined;
    private status: Status|undefined = undefined;
    private shape: egret.Shape = new egret.Shape();

    private x_: number;
    private y_: number;

    constructor(
        x: number,
        y: number,
        public owner: ICube,
        private readonly stage: egret.DisplayObjectContainer)
    {
        stage.addChild(this.shape);
        owner.entity.push(this);

        this.x_ = x;
        this.y_ = y;
        this.onPlace();
        this.onColor();
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
        this.moving = true;
    }

    commit(): void
    {
        if (this.modify === true) {
            this.onColor();
            this.modify = false;
        }

        if (this.moving === true) {
            if (this.action !== undefined && this.action !== Action.Idle &&
                this.status !== undefined && this.status !== Status.Stop)
            {
                const next = Action.toVec2(this.action).plus(this);
                switch(this.status) {
                case Status.Free:
                    this.x_ = next.x;
                    this.y_ = next.y;
                    this.onAnimationMove();
                    break;
                case Status.Lock:
                    this.onAnimationLock(next.x, next.y);
                    break;
                default:
                    this.onPlace();
                    break;
                }
            } else {
                this.onPlace();
            }

            this.action = undefined;
            this.status = undefined;
            this.moving = false;
        }
    }

    get x(): number
    {
        return this.x_;
    }

    get y(): number
    {
        return this.y_;
    }

    set x(value: number)
    {
        if (this.x !== value) {
            this.x_ = value;
            this.moving = true;
        }
    }

    set y(value: number)
    {
        if (this.x !== value) {
            this.y_ = value;
            this.moving = true;
        }
    }

    private onPlace(): void
    {
        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;

        const len = Math.min(wid / col, hgt / row);
        const tlx = (wid - col * len) / 2;
        const tly = (hgt - row * len) / 2;

        this.shape.x = tlx + this.x * len;
        this.shape.y = tly + this.y * len;
    }

    private onAnimationMove(): void
    {
        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;

        const len = Math.min(wid / col, hgt / row);
        const tlx = (wid - col * len) / 2;
        const tly = (hgt - row * len) / 2;

        const x = tlx + this.x * len;
        const y = tly + this.y * len;
        egret.Tween
            .get(this.shape)
            .to({x: x, y: y}, 250)
            ;
    }

    private onAnimationLock(x: number, y: number): void
    {
        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;

        const len = Math.min(wid / col, hgt / row);
        const tlx = (wid - col * len) / 2;
        const tly = (hgt - row * len) / 2;

        const src_x = tlx + this.x * len;
        const src_y = tly + this.y * len;
        const dst_x = (src_x + tlx + x * len) / 2;
        const dst_y = (src_y + tly + y * len) / 2;

        egret.Tween
            .get(this.shape)
            .to({x: dst_x, y: dst_y}, 125)
            .to({x: src_x, y: src_y}, 125)
            ;
    }

    private onColor(): void
    {
        const val = Unit.toColor(this.owner.type);

        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;
        const len = Math.min(wid / col, hgt / row);

        this.shape.graphics.clear();
        this.shape.graphics.beginFill(val);
        this.shape.graphics.drawRect(0, 0, len + 0.5, len + 0.5);
        this.shape.graphics.endFill();
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