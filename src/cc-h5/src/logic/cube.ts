namespace logic {
/////////////////////////////////////////////////////////////////////////////

export interface IWorld
{
    /** width of the grid. */
    readonly size: {
        readonly width: number;
        readonly height: number;
    };

    /** all cubes in the world. */
    readonly cube: ReadonlyArray<ICube>;

    /** all dest in the world. */
    readonly dest: ReadonlyArray<IVec2>;
}

/////////////////////////////////////////////////////////////////////////////

export interface ICube
{
    /** type of this cube. */
    readonly type: Cube.Type;

    /** does this cube have any entity. */
    readonly live: boolean;
    
    /** the world where this cube exises. */
    readonly world: IWorld;

    /** Does this cube is doing something.
     *  Note: Action.Idle is also "moving"
     */
    readonly moving: boolean;
    
    /** if it is able to absorb each others. */
    readonly active: boolean;
    
    /** all occupied positions. */
    readonly entity: Array<IUnit>;

    /** current cube's Behavior. */
    readonly behavior: IBehavior;

    /** set or get current action. */
    action: Cube.Action;

    /** set or get current status */
    status: Cube.Status;

    /** try to absorb others */
    absorb(others: Array<ICube>): void;

    /** can this absorb that? */
    absorbable(that: ICube): boolean;

    /** commit changes and perform some animation */
    commit(): void;
}

/////////////////////////////////////////////////////////////////////////////

export interface IDest extends IVec2
{
    // TODO:
}

/////////////////////////////////////////////////////////////////////////////

export interface IUnit extends IVec2
{
    /** transfer to target cube */
    attach(target: ICube): void;

    /** change an action with status */
    change(action: Cube.Action, status: Cube.Status): void;

    /** commit changes and perform some animation */
    commit(): void;
}

/////////////////////////////////////////////////////////////////////////////

export interface ICubeFactory
{
    create(src: Seed.Vec2): IVec2;
    create(src: Seed.Cube): ICube;
}

/////////////////////////////////////////////////////////////////////////////

export namespace Cube
{
    export enum Type
    {
        White,
        Green,
        Blue,
        Red,
    }

    export namespace Type
    {
        export function active(type: Type): boolean
        {
            switch(type) {
            case Type.Red: case Type.Blue: case Type.Green:
                return true;
            case Type.White: default:
                return false;
            }
        }

        export function absorbable(lhs: Type, rhs: Type): boolean
        {
            switch(lhs) {
            case Type.Red:
            {
                return rhs === Type.Red
                    || rhs === Type.Blue
                    || rhs === Type.Green
                    ;
            }
            case Type.Blue:
            {
                return rhs === Type.Blue
                    || rhs === Type.Green
                    ;
            }
            case Type.Green:
            {
                return rhs === Type.Green
                    ;
            }
            case Type.White:
            default:
            {
                return false;
            }
            }
        }
    }

    export enum Status
    {
        /** free to move */
        Free,
        /** compete for a position */
        Lock,
        /** blocked, cannot move */
        Stop,
    }

    export enum Action
    {
        Idle,
        Left,
        Down,
        Up,
        Right,
    }

    export namespace Action
    {
        export const Move: ReadonlyArray<Action> =
        [
            Action.Left, Action.Down, Action.Up, Action.Right
        ];

        export const Enum: ReadonlyArray<Action> =
        [
            Action.Idle, Action.Left, Action.Down, Action.Up, Action.Right
        ];

        export function toVec2(src: Action): Vec2
        {
            switch (src) {
                default:
                case Action.Idle : return Vec2.Zero;
                case Action.Left : return Vec2.Left;
                case Action.Down : return Vec2.Down;
                case Action.Up   : return Vec2.Up;
                case Action.Right: return Vec2.Right;
            }
        }

        export function opposite(src: Action): Action
        {
            switch (src) {
                default          : return Action.Idle;
                case Action.Left : return Action.Right;
                case Action.Down : return Action.Up;
                case Action.Up   : return Action.Down;
                case Action.Right: return Action.Left;
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
}