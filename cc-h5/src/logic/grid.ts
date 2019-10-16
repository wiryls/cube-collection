namespace logic {
/////////////////////////////////////////////////////////////////////////////

abstract class GridBase<T>
{

    constructor(readonly width: number, readonly height: number)
    { }

    protected abstract getRaw(p: IVec2): T;

    protected abstract setRaw(v: IVec2, val: T): void;

    protected hasOne(v: IVec2): boolean
    {
        return 0 <= v.x && v.x < this.width
            && 0 <= v.y && v.y < this.height
            ;
    }

    protected hasAll(vs: ReadonlyArray<IVec2>): boolean
    {
        return vs.every(v => this.hasOne(v));
    }

    protected setOne(v: IVec2, val: T): boolean
    {
        const has = this.hasOne(v);
        if (has)
            this.setRaw(v, val);
        return has;
    }

    protected setAll(vs: ReadonlyArray<IVec2>, val: T): boolean
    {
        return vs
            .map  (v => this.setOne(v, val))
            .every(b => b)
            ;
    }

    protected getAll(vs: ReadonlyArray<IVec2>): Array<T>
    {
        return vs
            .filter(v => this.hasOne(v))
            .map   (v => this.getRaw(v))
            ;
    }
}

/////////////////////////////////////////////////////////////////////////////

abstract class Uint8Grid extends GridBase<number>
{
    private grid: Uint8Array;

    constructor(width: number, height: number)
    {
        super(width, height);
        this.grid = new Uint8Array(width * height);
    }

    clear(): void
    {
        this.grid.fill(0);
    }

    protected getOne(v: IVec2): number
    {
        return this.hasOne(v) ? this.getRaw(v): 0;
    }

    protected getRaw(v: IVec2): number
    {
        return this.grid[this.index(v)];
    }

    protected setRaw(v: IVec2, val: number): void
    {
        this.grid[this.index(v)] = val;
    }

    private index(v: IVec2): number {
        return v.x + v.y * this.width;
    }
}

/////////////////////////////////////////////////////////////////////////////

export class ItemGrid<T> extends GridBase<T|undefined>
{
    private readonly grid: Array<T|undefined>;

    constructor(width: number, height: number)
    {
        super(width, height);
        this.grid = new Array<T|undefined>(width * height);
    }

    has(v: IVec2|ReadonlyArray<IVec2>): boolean
    {
        return(Guard.isIVec2(v))
            ? (this.hasOne(v))
            : (this.hasAll(v))
            ;
    }

    get(v :               IVec2 ): T|undefined;
    get(vs: ReadonlyArray<IVec2>): Array<T>;
    get(vs: ReadonlyArray<IVec2>|IVec2): Array<T>|T|undefined
    {
        if (Guard.isIVec2(vs))
            return this.getOne(vs);

        const rv = vs
            .map(v => this.getOne(v))
            .filter((c): c is T => c !== undefined)
            .map(c => c as T)
            ;

        return(rv.length <= 1)
            ? (rv)
            : (Array.from(new Set<T>(rv)))
            ;
    }

    set(v :               IVec2 ,       val: T): boolean;
    set(vs: ReadonlyArray<IVec2>,       val: T): boolean;
    set(vs: ReadonlyArray<IVec2>|IVec2, val: T): boolean
    {
        if (Guard.isIVec2(vs))
            return this.setOne(vs, val);
        else
            return this.setAll(vs, val);
    }

    clear(): void
    {
        this.grid.fill(undefined);
    }

    protected getOne(v: IVec2): T|undefined
    {
        return this.hasOne(v) ? this.getRaw(v) : undefined;
    }

    protected getRaw(v: IVec2): T|undefined
    {
        return this.grid[this.index(v)];
    }

    protected setRaw(v: IVec2, c: T): void
    {
        this.grid[this.index(v)] = c;
    }

    private index(v: IVec2): number
    {
        return v.x + v.y * this.width;
    }
}

/////////////////////////////////////////////////////////////////////////////

enum Edge
{
    L = 0b1000,
    D = 0b0100,
    U = 0b0010,
    R = 0b0001,
}

namespace Edge
{   
    export const Bits = 4;
    export const Mask = (1 << Bits) - 1;

    export const Enum : ReadonlyArray<Edge> =
    [
        Edge.L, Edge.D, Edge.U, Edge.R
    ];

    export function is(src: number, dst: Edge): boolean
    {
        return (src & dst) === dst;
    }

    export function opposite(src: Edge): Edge
    {
        switch (src) {
            default:     return src;
            case Edge.L: return Edge.R;
            case Edge.D: return Edge.U;
            case Edge.U: return Edge.D;
            case Edge.R: return Edge.L;
        }
    }

    export function ccw(src: Edge): Edge
    {
        switch (src) {
            default:     return src;
            case Edge.L: return Edge.D;
            case Edge.D: return Edge.R;
            case Edge.U: return Edge.L;
            case Edge.R: return Edge.U;
        }
    }

    export function cw(src: Edge): Edge
    {
        switch (src) {
            default:     return src;
            case Edge.L: return Edge.U;
            case Edge.D: return Edge.L;
            case Edge.U: return Edge.R;
            case Edge.R: return Edge.D;
        }
    }

    export function toVec2(src: Edge): Vec2
    {
        switch (src) {
            // default
            default:     return Vec2.Zero;
            // horizontal or vertical
            case Edge.L: return Vec2.Left;
            case Edge.D: return Vec2.Down;
            case Edge.U: return Vec2.Up;
            case Edge.R: return Vec2.Right;
            // diagonal
            case Edge.L|Edge.U: return Vec2.UpLeft;
            case Edge.L|Edge.D: return Vec2.DownLeft;
            case Edge.R|Edge.U: return Vec2.UpRight;
            case Edge.R|Edge.D: return Vec2.DownRight;
        }
    }

    export function fromAction(src: Cube.Action): ReadonlyArray<Edge>
    {
        switch (src) {
            default:                return Enum;
            case Cube.Action.Left : return [Edge.L];
            case Cube.Action.Down : return [Edge.D];
            case Cube.Action.Up   : return [Edge.U];
            case Cube.Action.Right: return [Edge.R];
        }
    }
}

export class EdgeGrid extends Uint8Grid
{
    constructor(width: number, height: number)
    {
        super(width, height);
    }

    get(v: IVec2): number
    {
        return this.getOne(v);
    }

    put(vs: ReadonlyArray<IVec2>): void
    {
        for (const v of vs) {
            let s = 0;
            for(const e of Edge.Enum) {
                const nv = Edge.toVec2(e).plus(v);
                const ns = this.getOne(nv) >> Edge.Bits;
                if (ns !== 0) {
                    this.setOne(nv, (ns & ~Edge.opposite(e)) << Edge.Bits);
                } else {
                    s |= e << Edge.Bits;
                }
            }
            this.setOne(v, s);
        }

        for (const v of vs)
            this.setOne(v, this.getOne(v) >> Edge.Bits);
    }

    out(vs: ReadonlyArray<IVec2>, a: Cube.Action = Cube.Action.Idle): Array<Vec2>
    {
        const rs = new Array<Vec2>();
        const es = Edge.fromAction(a);
        for(const v of vs) {
            const s = this.getOne(v);
            for (const e of es)
                if (Edge.is(s, e))
                    rs.push(Edge.toVec2(e).plus(v));
        }

        return rs;
    }

    cor(vs: ReadonlyArray<IVec2>, a: Cube.Action = Cube.Action.Idle): Array<Vec2>
    {
        const rs = new Array<Vec2>();
        const es = Array.from(Edge.fromAction(a).reduce(
            (s, e) => s.add(e | Edge.cw(e)) && s.add(e | Edge.ccw(e)), new Set<Edge>()
        ));

        for(const v of vs) {
            const s = this.getOne(v);
            for (const e of es)
                if (Edge.is(s, e))
                    rs.push(Edge.toVec2(e).plus(v));
        }

        return rs
    }
}

/////////////////////////////////////////////////////////////////////////////

namespace Guard
{
    export function isIVec2<T>(v: IVec2|T): v is IVec2
    {
        return (<IVec2>v).x !== undefined
            && (<IVec2>v).y !== undefined
            ;
    }
}

/////////////////////////////////////////////////////////////////////////////
}