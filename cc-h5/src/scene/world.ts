namespace scene {
/////////////////////////////////////////////////////////////////////////////

const Track = utils.Track;
const Musician = utils.Musician.instance;

/////////////////////////////////////////////////////////////////////////////

export class World extends eui.Component
{
    private input: ReadonlyArray<input.Controller>;
    private order: Array<input.Controller.Type> = new Array<input.Controller.Type>(3);

    private guide: logic.Narrator = new logic.Narrator(new utils.Loader(), new utils.Saver());
    private world: entity.World = new entity.World();
    private timer: egret.Timer = new egret.Timer(250, 0);

    public constructor()
    {
        super();
        this.addEventListener(egret.Event.ADDED_TO_STAGE, this.onAddToStage, this);
        this.input = [
            input.Controller.create(this),
            input.Controller.create(new input.KeyBoard()),
        ];
    }

    private onAddToStage(): void
    {
        // background
        const shape = new egret.Shape();
        this.addChild(shape);
        shape.graphics.beginFill(0x000000);
        shape.graphics.drawRect(0, 0, this.stage.stageWidth, this.stage.stageHeight);
        shape.graphics.endFill();

        // world
        const seed = this.guide.tell();
        if (seed === undefined)
            throw new Error("Cannot find any Seed");

        this.addChild(this.world);
        this.world.build(seed);

        // begin
        for (const i of this.input) {
            i.enable = true;
            i.addEventListener(input.Controller.Event.ORDER, this.onControl, this);
        }

        this.timer.start();
        this.timer.addEventListener(egret.TimerEvent.TIMER, this.onTick, this);
    }

    private onTick(event:egret.Event): void
    {
        if (this.world === undefined)
            return;

        const idle = input.Controller.Type.MOVE_IDLE;
        if (this.order.some(o => o !== idle))
        {
            // this.order[0]: last valid order
            // this.order[1]: current valid order
            // this.order[2]: current order
            const o
                = this.order[0] === idle
                ? this.order[1]
                : this.order[2] === idle && this.order[1] === this.order[0]
                ? this.order[2]
                : this.order[1]
                ;

            if (o !== idle) {
                this.world.command(o as number);
            } else {
                this.order[1] = o;
            }
            this.order[0] = o;
        }

        this.world.next();
    }

    private onControl(event: input.Controller.Event): void
    {
        // console.log("Control", event.code);
        switch(event.code) {
        case input.Controller.Type.CTRL_EXIT:
        {
            const seed = this.guide.zero();
            if (seed !== undefined)
                this.world.build(seed);
            break;    
        }
        case input.Controller.Type.CTRL_RESTART:
        {
            this.world.build();
            break;
        }
        case input.Controller.Type.CTRL_DONE:
        {
            const flag = this.world.status();
            if (flag.some(n => n !== 0)) {
                const seed = this.guide.next(flag);
                if (seed !== undefined) {
                    Musician.sound(Track.LEVEL_ENTER);
                    this.world.build(seed);
                }
            }
            break;
        }
        case input.Controller.Type.CTRL_SKIP:
        {
            // used for debug
            const seed = this.guide.next(this.world.status());
            if (seed !== undefined) {
                Musician.sound(Track.LEVEL_ENTER);
                this.world.build(seed);
            }
            break;
        }
        default:
        {
            if (input.Controller.Moves.includes(event.code)) {
                if (event.code !== input.Controller.Type.MOVE_IDLE) {
                    Musician.sound(Track.CUBE_CONTROL);
                    this.order[1] = event.code;
                }
                this.order[2] = event.code;
            }
            break;
        }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
}