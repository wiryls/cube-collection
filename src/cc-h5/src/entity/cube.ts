namespace entity {
/////////////////////////////////////////////////////////////////////////////

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

        // color and shape
        if (this.modify_) {
            for (const e of this.entity_)
                e.attach(this);

            for (const e of this.entity)
                e.commit();

            this.modify_ = false;
        }

        // move
        if (this.moving) {
            for (const e of this.entity_)
                e.change(this.action, this.status);

            for (const e of this.entity)
                e.commit();

            this.status = Status.Free;
            this.behavior.next();
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

class Unit implements IUnit
{
    private modify: boolean = false;
    private moving: boolean = false;

    private action: Action|undefined = undefined;
    private status: Status|undefined = undefined;

    private parts: Array<egret.Shape>;

    private x_: number;
    private y_: number;

    constructor(
        x: number,
        y: number,
        public owner: ICube,
        private readonly stage: egret.DisplayObjectContainer)
    {
        this.parts = [new egret.Shape(), new egret.Shape()];
        layer[1].addChild(this.parts[0]);
        layer[3].addChild(this.parts[1]);
        owner.entity.push(this);

        this.x_ = x;
        this.y_ = y;
        this.onPaint();
        this.onPlace();
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
            this.onPaint();
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

        this.cube_top.x = tlx + this.x * len;
        this.cube_btm.x = tlx + this.x * len;
        this.cube_top.y = tly + this.y * len;
        this.cube_btm.y = tly + this.y * len;
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

        for (const part of this.parts) {
            // make a smaller displacement for sort
            part.x += (part.x < x) ? +1 : (part.x > x) ? -1 : 0;
            part.y += (part.y < y) ? +1 : (part.y > y) ? -1 : 0;
            // animation
            egret.Tween
                .get(part)
                .to({x: x, y: y}, 240)
                ;
        }
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

        for (const part of this.parts) {
            // make a smaller displacement for sort
            part.x += (part.x < dst_x) ? +1 : (part.x > dst_x) ? -1: 0;
            part.y += (part.y < dst_y) ? +1 : (part.y > dst_y) ? -1: 0;
            // animation
            egret.Tween
                .get(part)
                .to({x: dst_x, y: dst_y}, 120, egret.Ease.sineOut)
                .to({x: src_x, y: src_y}, 120, egret.Ease.sineIn)
                ;
        }
    }

    private onPaint(): void
    {
        const col = this.owner.world.size. width;
        const row = this.owner.world.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;
        const len = Math.min(wid / col, hgt / row);

        for (const part of this.parts)
            part.graphics.clear();

        let flag = 0;
        for (const v of this.owner.entity) {
            if (v.x === this.x_ - 1 && v.y === this.y_)
                flag |= 1; // left
            else if (v.x === this.x_ && v.y === this.y_ - 1)
                flag |= 2; // up
        }
        
        setColor(this.parts[0], this.parts[1], len, this.owner.type, flag);
    }
}

/////////////////////////////////////////////////////////////////////////////

export class Dest implements IVec2
{
    private parts: Array<egret.Shape>;

    constructor(
        public readonly x: number,
        public readonly y: number,
        private readonly owner: IWorld,
        private readonly stage: egret.DisplayObjectContainer)
    {
        this.parts = [new egret.Shape(), new egret.Shape()];
        layer[0].addChild(this.parts[0]);
        layer[2].addChild(this.parts[1]);

        const col = this.owner.size. width;
        const row = this.owner.size.height;
        const wid = this.stage.stage.stageWidth;
        const hgt = this.stage.stage.stageHeight;

        const len = Math.min(wid / col, hgt / row);
        const tlx = (wid - col * len) / 2;
        const tly = (hgt - row * len) / 2;

        setColor(this.parts[0], this.parts[1], len);
        for (const part of this.parts) {
            part.x = tlx + this.x * len;
            part.y = tly + this.y * len;
            part.alpha = 0.4;
            egret.Tween
                .get(part, {loop: true})
                .to({alpha: 0.2}, 2000, egret.Ease.sineInOut)
                .to({alpha: 0.4}, 2000, egret.Ease.sineInOut)
                ;
        }

    }
}

/////////////////////////////////////////////////////////////////////////////

export class CubeFactory
{
    constructor(
        private readonly world: IWorld,
        private readonly stage: egret.DisplayObjectContainer)
    {}

    create(src: Seed.Vec2): Dest;
    create(src: Seed.Cube): Cube;
    create(src: Seed.Cube|Seed.Vec2): Cube|Dest
    {
        if (Array.isArray(src)) {
            const dest = new Dest(src[0], src[1], this.world, this.stage);
            return dest;
        } else {
            const type = Seed.Cube.toType(src.type);
            const move
                = (src.move === undefined)
                ? (Behavior.create())
                : (Behavior.create(src.move.loop, Seed.Cube.toActions(src.move.path)))
                ;
            const cube = new Cube(this.world, type, move);

            for (const v of src.body)
                new Unit(v[0], v[1], cube, this.layer).attach(cube);

            for (const e of cube.entity)
                e.commit();

            return cube;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

function setColor(top: egret.Shape, bottom: egret.Shape, size: number, type: Type = Type.White, style: number = 0): void
{
    const color = [0xFFFFFF, 0xFFFFFF];    
    // color
    switch(type) {
    default:
    case Type.White: color[0] = 0xFFFFFF; color[1] = 0xD0D0D0; break; // White 0xEAE9E8;
    case Type.Green: color[0] = 0xBBDBB9; color[1] = 0x6F9A6D; break; // Green 0x75FF81;
    case Type.Blue : color[0] = 0xAED3F1; color[1] = 0x6894B7; break; // Blue  0x75C6FF;
    case Type.Red  : color[0] = 0xFC9C9C; color[1] = 0xCB5B5B; break; // Red   0xF33048;
    }

    // top
    const gap = size * 0.03;
    const rect = [gap, gap, size-gap*0.5, size-gap*0.5];

    top.graphics.beginFill(color[0]);
    top.graphics.drawRect(rect[0], rect[1], rect[2], rect[3]);
    if ((style & 1 /* left */) !== 0)
        top.graphics.drawRect(rect[0]-gap, rect[1], gap*2, rect[3]);
    if ((style & 2 /* up   */) !== 0)
        top.graphics.drawRect(rect[0], rect[1]-gap, rect[2], gap*2);
    top.graphics.endFill();

    const len = [gap*0.5, size / 6, size, size * 7 / 6];

    bottom.graphics.beginFill(color[1]);

    bottom.graphics.moveTo(len[2], len[0]);
    bottom.graphics.lineTo(len[3], len[1]);
    bottom.graphics.lineTo(len[3], len[3]);
    bottom.graphics.lineTo(len[2], len[2]);
    bottom.graphics.lineTo(len[2], len[0]);

    bottom.graphics.moveTo(len[0], len[2]);
    bottom.graphics.lineTo(len[2], len[2]);
    bottom.graphics.lineTo(len[3], len[3]);
    bottom.graphics.lineTo(len[1], len[3]);
    bottom.graphics.lineTo(len[0], len[2]);

    bottom.graphics.endFill();
}

/////////////////////////////////////////////////////////////////////////////
}