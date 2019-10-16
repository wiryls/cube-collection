namespace logic {
/////////////////////////////////////////////////////////////////////////////

import Action = Cube.Action;

/////////////////////////////////////////////////////////////////////////////

export interface IBehavior
{
    /** if it is stopped */
    readonly done: boolean;

    /** set or get current action */
    action: Action;

    /** next action */
    next(): this;
}

export namespace Behavior
{
    type Actions   = Array<[Action, number]>;
    type Behaviors = Array<IBehavior>;

    export function create(): IBehavior;
    export function create(others: Behaviors): IBehavior;
    export function create(isloop: boolean, actions: Actions): IBehavior;
    export function create(fst?: boolean|Behaviors, snd?: Actions): IBehavior
    {
        if (fst === undefined)
            return new Idle();
        if (snd instanceof Array && typeof fst === 'boolean')
            return new Move(fst, snd);
        if (typeof fst === 'boolean')
            return None;

        fst = fst.filter(m => m.done === false);
        if (fst.length === 0)
            return new Idle();
        else if (fst.length === 1)
            return fst[0];
        else
            return new Moves(fst);
    }

    function toAction(src: string): Action
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
}

/////////////////////////////////////////////////////////////////////////////

class Dead implements IBehavior
{
    get action(): Action
    {
        return Action.Idle;
    }

    get done(): boolean
    {
        return true;
    }

    next (): this
    {
        return this;
    }
}

class Idle implements IBehavior
{
    private cache: Action | undefined = undefined;

    set action(action: Action)
    {
        this.cache = action;
    }

    get action(): Action
    {
        return(this.cache === undefined)
            ? (Action.Idle)
            : (this.cache)
            ;
    }

    get done(): boolean
    {
        return this.cache === undefined;
    }

    next(): this
    {
        this.cache = undefined;
        return this;
    }
}

class Move implements IBehavior
{
    private cache: Action | undefined = undefined;
    private count: number = 0;
    private index: number = 0;

    constructor(
        public readonly isloop: boolean,
        public readonly sequence: Array<[Action, number]>)
    {
        this.sequence = sequence.filter(v => v[1] > 0);
    }

    set action(action: Action)
    {
        this.cache = action;
    }

    get action(): Action
    {
        return(this.done)
            ? (Action.Idle)
            : (this.cache !== undefined)
            ? (this.cache)
            : (this.sequence[this.index][0])
            ;
    }

    get done(): boolean
    {
        return this.cache === undefined
            && this.index === this.sequence.length
            ;
    }

    next(): this
    {
        if (this.cache !== undefined) {
            this.cache = undefined;
            return this;
        }

        const iend = this.sequence.length;
        if (this.index === iend)
            return this;

        const cend = this.sequence[this.index][1];
        this.count += 1;
        if (this.count === cend) {
            this.index += 1;
            if (this.index === iend)
                if (this.isloop)
                    this.index = 0;
            this.count = 0;
        }

        return this;
    }
}

class Moves implements IBehavior
{
    private cache: Action|undefined = undefined;

    constructor(private moves: Array<IBehavior>)
    {
        if (this.moves.length === 0)
            this.moves.push(new Idle());
    }

    set action(action: Action)
    {
        this.moves.forEach(m => m.action = action);
    }

    get action(): Action
    {
        if (this.cache === undefined) {
            const maybe = this.moves[0].action;
            this.cache
                = (this.moves.every(o => o.action === maybe))
                ? (maybe)
                : (Action.Idle)
                ;
        }
        return this.cache;
    }

    get done(): boolean
    {
        return this.moves.every(m => m.done);
    }

    next(): this
    {
        this.cache = undefined;
        this.moves.forEach(m => m.next());
        this.moves = this.moves.filter((m, i) => i === 0 || m.done === false);
        return this;
    }
}

/////////////////////////////////////////////////////////////////////////////

export namespace Behavior
{
    export const None = new Dead();
}

/////////////////////////////////////////////////////////////////////////////
}