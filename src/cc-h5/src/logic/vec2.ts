namespace logic {
/////////////////////////////////////////////////////////////////////////////

export interface IVec2
{
    x: number;
    y: number;
}

export class Vec2
{
    private mem = new Int32Array(2);

    get x(): number  { return this.mem[0]; }
    set x(v: number) { this.mem[0] = v;    }
    get y(): number  { return this.mem[1]; }
    set y(v: number) { this.mem[1] = v;    }
    get 0(): number  { return this.mem[0]; }
    set 0(v: number) { this.mem[0] = v;    }
    get 1(): number  { return this.mem[1]; }
    set 1(v: number) { this.mem[1] = v;    }
    get length() { return 2; }

    constructor(v: Array<number>);
    constructor(x: number, y: number);
    constructor(v: number|Array<number>, y: number = 0)
    {
        if (v instanceof Array) {
            this[0] = Number(v[0]);
            this[1] = Number(v[1]);
        } else {
            this[0] = v;
            this[1] = y;
        }
    }

    plus(that: IVec2): this
    {
        this.x += that.x;
        this.y += that.y;
        return this;
    }

    static get Zero () { return new Vec2( 0,  0); }
    static get Left () { return new Vec2(-1,  0); }
    static get Down () { return new Vec2( 0,  1); }
    static get Up   () { return new Vec2( 0, -1); }
    static get Right() { return new Vec2( 1,  0); }
}

/////////////////////////////////////////////////////////////////////////////
}
