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
        CTRL_SKIP,
    }

    export const Moves : ReadonlyArray<Type> =
    [
        Type.MOVE_IDLE, Type.MOVE_LEFT, Type.MOVE_DOWN,
        Type.MOVE_UP, Type.MOVE_RIGHT,
    ];

    export const Ctrls : ReadonlyArray<Type> =
    [
        Type.CTRL_EXIT, Type.CTRL_DONE, Type.CTRL_PAUSE,
        Type.CTRL_RESUME, Type.CTRL_RESTART, Type.CTRL_SKIP,
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
    export function create(src: egret.DisplayObjectContainer): Controller;
    export function create(src: input.KeyBoard|egret.DisplayObjectContainer): Controller
    {
        if (src instanceof input.KeyBoard) {
            return new KeyBoardController(src);
        } else if (src instanceof egret.DisplayObjectContainer) {
            return new TouchController(src);
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

        if (Controller.Moves.includes(o) && o !== Controller.Type.MOVE_IDLE) {
            switch (o) {
            case Controller.Type.MOVE_LEFT : this.force[0] = Math.max(this.force[0] - 1, -1); break;
            case Controller.Type.MOVE_DOWN : this.force[1] = Math.min(this.force[1] + 1, +1); break;
            case Controller.Type.MOVE_UP   : this.force[1] = Math.max(this.force[1] - 1, -1); break;
            case Controller.Type.MOVE_RIGHT: this.force[0] = Math.min(this.force[0] + 1, +1); break;
            }

            const m = this.toMove();
            if (m !== undefined)
                o = m;
        }

        this.dispatchEvent(new Controller.Event(o, Controller.Event.ORDER, true, true));
    }

    private onKeyUp(event: input.Key.Event): void
    {
        let o = this.mapper(event.key);
        if (o === undefined)
            return;

        if (Controller.Moves.includes(o) && o !== Controller.Type.MOVE_IDLE) {
            switch (o) {
            case Controller.Type.MOVE_LEFT : this.force[0] = Math.min(this.force[0] + 1, +1); break;
            case Controller.Type.MOVE_DOWN : this.force[1] = Math.max(this.force[1] - 1, -1); break;
            case Controller.Type.MOVE_UP   : this.force[1] = Math.min(this.force[1] + 1, +1); break;
            case Controller.Type.MOVE_RIGHT: this.force[0] = Math.max(this.force[0] - 1, -1); break;
            }
            o = this.toMove() || Controller.Type.MOVE_IDLE;
        }

        if (Controller.Moves.includes(o))
            this.dispatchEvent(new Controller.Event(o, Controller.Event.ORDER, true, true));
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
            [input.Key.SPACE, Controller.Type.CTRL_DONE   ],
            [input.Key.P    , Controller.Type.CTRL_PAUSE  ],
            [input.Key.R    , Controller.Type.CTRL_RESTART],
            [input.Key.N    , Controller.Type.CTRL_SKIP   ],
        ]);
        return map.get(key);
    }

     private toMove(): Controller.Type | undefined
     {
        let [x, y] = this.force;
        switch (true) {
        default                  : return undefined;
        case x ===  0 && y ===  0: return Controller.Type.MOVE_IDLE ;
        case x === -1 && y ===  0: return Controller.Type.MOVE_LEFT ;
        case x ===  0 && y ===  1: return Controller.Type.MOVE_DOWN ;
        case x ===  0 && y === -1: return Controller.Type.MOVE_UP   ;
        case x ===  1 && y ===  0: return Controller.Type.MOVE_RIGHT;
        }
    }
}

/////////////////////////////////////////////////////////////////////////////

class TouchController extends egret.EventDispatcher implements Controller
{
    private state: boolean = false;
    private moved: boolean = false;
    private point: [number, number] = [0, 0];
    private ctype: Controller.Type = Controller.Type.MOVE_IDLE;
    private begin: number = 0;

    constructor(private scene: egret.DisplayObjectContainer)
    {
        super();
    }

    set enable(state: boolean)
    {
        if (this.state === state)
            return;

        if (state) {
            this.scene.touchEnabled = true;
            this.scene.addEventListener(egret.TouchEvent.TOUCH_BEGIN, this.onTouchBegin, this);
            this.scene.addEventListener(egret.TouchEvent.TOUCH_END, this.onTouchEnd, this);
        } else {
            this.scene.removeEventListener(egret.TouchEvent.TOUCH_END, this.onTouchEnd, this);
            this.scene.removeEventListener(egret.TouchEvent.TOUCH_BEGIN, this.onTouchBegin, this);
            this.scene.touchEnabled = false;
            this.moved = false;
            this.ctype = Controller.Type.MOVE_IDLE;
        }

        this.state = state;
    }

    get enable(): boolean
    {
        return this.state;
    }

    private onTouchTap(event: egret.TouchEvent): void
    {
        const scale = Math.min(this.scene.stage.stageWidth, this.scene.stage.stageHeight);
        const x = Math.min(Math.max(event.stageX / scale, 0), 1); /* [0, 1] */
        const y = Math.min(Math.max(event.stageY / scale, 0), 1); /* [0, 1] */
        const type
            = (x < 0.1 && y < 0.1) ? Controller.Type.CTRL_RESTART
            : (x > 0.9 && y < 0.1) ? Controller.Type.CTRL_EXIT
            : Controller.Type.CTRL_DONE
            ;

        this.dispatchEvent(new Controller.Event(type, Controller.Event.ORDER, true, true));
    }

    private onTouchBegin(event: egret.TouchEvent): void
    {
        this.scene.stage.addEventListener(egret.TouchEvent.TOUCH_MOVE, this.onTouchMove, this);
        this.point = [event.stageX, event.stageY];
        this.begin = Date.now();
    }

    private onTouchEnd(event: egret.TouchEvent): void
    {
        this.scene.stage.removeEventListener(egret.TouchEvent.TOUCH_MOVE, this.onTouchMove, this);

        if (Date.now() - this.begin < 500 && this.moved === false) {
            this.onTouchTap(event);
        } else {
            this.moved = false;
            this.ctype = Controller.Type.MOVE_IDLE;
            this.dispatchEvent(new Controller.Event(this.ctype, Controller.Event.ORDER, true, true));
        }
    }

    private onTouchMove(event: egret.TouchEvent): void
    {
        let type = Controller.Type.MOVE_IDLE;

        const scale = Math.min(this.scene.stage.stageWidth, this.scene.stage.stageHeight) / 2;
        const dx = this.point[0] - event.stageX;
        const dy = this.point[1] - event.stageY;
        const x = Math.min(Math.max(dx / scale, -1), +1);
        const y = Math.min(Math.max(dy / scale, -1), +1);

        let change = true;
        switch (true) {
        case Math.abs(y) < Math.abs(x) && 0.1 < Math.abs(x):
            type = x > 0 ? Controller.Type.MOVE_LEFT : Controller.Type.MOVE_RIGHT;
            this.moved = true;
            break;
        case Math.abs(x) < Math.abs(y) && 0.1 < Math.abs(y):
            type = y > 0 ? Controller.Type.MOVE_UP : Controller.Type.MOVE_DOWN;
            this.moved = true;
            break;
        default:
            change = false;
            break;
        }

        if (this.moved && change) {
            this.point = [event.stageX, event.stageY];
            if (this.ctype !== type) {
                this.ctype = type;
                this.dispatchEvent(new Controller.Event(type, Controller.Event.ORDER, true, true));
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
}