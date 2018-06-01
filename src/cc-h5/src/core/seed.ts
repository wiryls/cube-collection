namespace core {
/////////////////////////////////////////////////////////////////////////////

export interface Seed
{
    head: Seed.Head;
    size: Seed.Size;
    cube: Array<Seed.Cube>;
    dest: Array<Seed.Vec2>;
}

export namespace Seed
{
    export interface Head
    {
        title: string;
        [ key: string]: string;
    }

    export interface Size
    {
         width: number;
        height: number;
    }

    export interface Vec2 extends Array<number>
    {
        /** x */
        0: number;
        /** y */
        1: number;
    }

    export interface Cube
    {
        type: "W"|"R"|"G"|"B";
        body: Array<Vec2>;
        move: Cube.Move|undefined;
    }

    export namespace Cube
    {
        export interface Move
        {
            loop: boolean;
            path: Path;
        }

        export interface Path extends Array<["I"|"L"|"D"|"U"|"R", number]>
        {}
    }
}

/////////////////////////////////////////////////////////////////////////////

export namespace Seed.Cube
{
    export import Type   = core.Cube.Type;
    export import Action = core.Cube.Action;

    export function toAction(src: string): Action
    {
        switch (src) {
            default:
            case "I": case "i": return Action.Idle;
            case "L": case "l": return Action.Left;
            case "D": case "d": return Action.Down;
            case "U": case "u": return Action.Up;
            case "R": case "r": return Action.Right;
        }
    }

    export function toActions(src: Path): Array<[Action, number]>
    {
        return src.map(([a, i]) => [toAction(a), i] as [Action, number]);
    }

    export function toType(src: string): Type
    {
        switch (src) {
            default:
            case "W": case "w": return Type.White;
            case "G": case "g": return Type.Green;
            case "B": case "b": return Type.Blue;
            case "R": case "r": return Type.Red;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

export namespace Guard
{
    export function isSeed(it: any): it is Seed
    {
        const its = it as Seed;
        return isHead(its.head)
            && isSize(its.size)
            && isCube(its.cube)
            && isDest(its.dest)
            ;
    }

    function isHead(it: any): it is Seed.Head
    {
        const its = it as Seed.Head;
        return typeof its.title === 'string';
    }

    function isSize(it: any): it is Seed.Size
    {
        const its = it as Seed.Size;
        return typeof its. width === 'number'
            || typeof its.height === 'number'
            ;
    }

    function isCube(it: any): it is Array<Seed.Cube>
    {
        return Array.isArray(it);
    }

    function isDest(it: any): it is Array<Seed.Vec2>
    {
        return Array.isArray(it);
    }
}

/////////////////////////////////////////////////////////////////////////////
}