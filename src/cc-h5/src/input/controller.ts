namespace input {
/////////////////////////////////////////////////////////////////////////////

export interface Controller extends egret.EventDispatcher
{
    enable: boolean;
}

export namespace Controller
{
    export const enum Type
    {
        MOVE_IDLE,
        MOVE_LEFT,
        MOVE_DOWN,
        MOVE_UP,
        MOVE_RIGHT,

        CTRL_EXIT,
        CTRL_DONE,
        CTRL_PAUSE,
        CTRL_RESUME = CTRL_PAUSE,
        CTRL_RESTART,
    }

    export const Moves : ReadonlyArray<Type> =
    [
        Type.MOVE_IDLE, Type.MOVE_LEFT, Type.MOVE_DOWN,
        Type.MOVE_UP, Type.MOVE_RIGHT,
    ];

    export const Ctrls : ReadonlyArray<Type> =
    [
        Type.CTRL_EXIT, Type.CTRL_DONE, Type.CTRL_PAUSE,
        Type.CTRL_RESUME, Type.CTRL_RESTART,
    ];

    export class Event extends egret.Event
    {
        static readonly ORDER = "Order.Event.COMMAND";

        constructor(
            public readonly code: Type,
			type      : string,
			bubbles   : boolean = false,
			cancelable: boolean = false)
		{
			super(type, bubbles, cancelable);
		}
    }

    export function create(src: input.KeyBoard): Controller;
    export function create(src: input.KeyBoard): Controller
    {
        if (src instanceof input.KeyBoard) {
            return new KeyBoardController(src);
        } else {
            return new DummyController();
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

class DummyController extends egret.EventDispatcher implements Controller
{
    readonly enable: boolean = false;
}

/////////////////////////////////////////////////////////////////////////////

class KeyBoardController extends egret.EventDispatcher implements Controller
{
    private state: boolean = false;
    private force: [number, number] = [0, 0];

    // TODO: Add a key mapper to avoid hardcode.
    constructor(private input: input.KeyBoard)
    {
        super();
    }

    set enable(state: boolean)
    {
        if (this.state === state)
            return;

        if (state) {
            this.input.addEventListener(input.Key.Event.KEY_DOWN, this.onKeyDown, this);
            this.input.addEventListener(input.Key.Event.KEY_UP,   this.onKeyUp,   this);
        } else {
            this.input.removeEventListener(input.Key.Event.KEY_DOWN, this.onKeyDown, this);
            this.input.removeEventListener(input.Key.Event.KEY_UP,   this.onKeyUp,   this);
        }

        this.state = state;        
    }

    get enable(): boolean
    {
        return this.state;
    }

    private onKeyDown(event: input.Key.Event): void
    {
        let o = this.mapper(event.key);
        if (o === undefined)
            return;
        
        if (Controller.Moves.includes(o) && o != Controller.Type.MOVE_IDLE) {
            switch (o) {
            case Controller.Type.MOVE_LEFT : this.force[0] -= 1; break;
            case Controller.Type.MOVE_DOWN : this.force[1] += 1; break;
            case Controller.Type.MOVE_UP   : this.force[1] -= 1; break;
            case Controller.Type.MOVE_RIGHT: this.force[0] += 1; break;
            }
            o = this.toMove();
        }

        const e = new Controller.Event(o, Controller.Event.ORDER, true, true);
        this.dispatchEvent(e);
    }

    private onKeyUp(event: input.Key.Event): void
    {
        let o = this.mapper(event.key);
        if (o === undefined)
            return;
        
        if (Controller.Moves.includes(o) && o != Controller.Type.MOVE_IDLE) {
            switch (o) {
            case Controller.Type.MOVE_LEFT : this.force[0] += 1; break;
            case Controller.Type.MOVE_DOWN : this.force[1] -= 1; break;
            case Controller.Type.MOVE_UP   : this.force[1] += 1; break;
            case Controller.Type.MOVE_RIGHT: this.force[0] -= 1; break;
            }
            o = this.toMove();
        }

        const e = new Controller.Event(o, Controller.Event.ORDER, true, true);
        this.dispatchEvent(e);
    }

    private mapper(key: input.Key): Controller.Type|undefined
    {
        const map = new Map<input.Key, Controller.Type>([
            // move
            [input.Key.LEFT , Controller.Type.MOVE_LEFT ],
            [input.Key.DOWN , Controller.Type.MOVE_DOWN ],
            [input.Key.UP   , Controller.Type.MOVE_UP   ],
            [input.Key.RIGHT, Controller.Type.MOVE_RIGHT],
            // ctrl
            [input.Key.ESC  , Controller.Type.CTRL_EXIT   ],
            [input.Key.ENTER, Controller.Type.CTRL_DONE   ],
            [input.Key.P    , Controller.Type.CTRL_PAUSE  ],
            [input.Key.R    , Controller.Type.CTRL_RESTART],
        ]);
        return map.get(key);
    }

     private toMove(): Controller.Type
     {
        let [x, y] = this.force;
        switch (true) {
        default                  :
        case x ===  0 && y ===  0: return Controller.Type.MOVE_IDLE ;
        case x === -1 && y ===  0: return Controller.Type.MOVE_LEFT ;
        case x ===  0 && y ===  1: return Controller.Type.MOVE_DOWN ;
        case x ===  0 && y === -1: return Controller.Type.MOVE_UP   ;
        case x ===  1 && y ===  0: return Controller.Type.MOVE_RIGHT;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
}