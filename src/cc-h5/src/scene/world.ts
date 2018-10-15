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
        const shape = new egret.Shape();
        this.addChild(shape);
        shape.graphics.beginFill(0x000000);
        shape.graphics.drawRect(0, 0, this.stage.stageWidth, this.stage.stageHeight);
        shape.graphics.endFill();

        const raws = <logic.Seed><any>{"size":{"height":11,"width":20},"head":{"author":"MYLS","title":"Debug"},"dest":[[18,5]],"version":1,"cube":[{"body":[[0,0],[1,0],[2,0],[3,0],[4,0],[5,0],[6,0],[7,0],[8,0],[9,0],[10,0],[11,0],[12,0],[13,0],[14,0],[15,0],[16,0],[17,0],[18,0],[19,0],[0,1],[19,1],[0,2],[19,2],[0,3],[19,3],[0,4],[19,4],[0,5],[19,5],[0,6],[19,6],[0,7],[19,7],[0,8],[11,8],[12,8],[14,8],[15,8],[19,8],[0,9],[19,9],[0,10],[1,10],[2,10],[3,10],[4,10],[5,10],[6,10],[7,10],[8,10],[9,10],[10,10],[11,10],[12,10],[13,10],[14,10],[15,10],[16,10],[17,10],[18,10],[19,10]],"type":"W"},{"body":[[3,9]],"type":"B"},{"body":[[3,1]],"type":"R","move":{"path":[["R",1]],"loop":true}},{"body":[[5,1]],"type":"R","move":{"path":[["L",1]],"loop":true}},{"body":[[10,1]],"type":"W","move":{"path":[["R",1]],"loop":true}},{"body":[[12,1]],"type":"W","move":{"path":[["L",1]],"loop":true}},{"body":[[3,2]],"type":"R","move":{"path":[["R",1]],"loop":true}},{"body":[[6,2]],"type":"R","move":{"path":[["L",1]],"loop":true}},{"body":[[10,2]],"type":"W","move":{"path":[["R",1]],"loop":true}},{"body":[[13,2]],"type":"W","move":{"path":[["L",1]],"loop":true}},{"body":[[3,4]],"type":"G","move":{"path":[["R",1]],"loop":true}},{"body":[[5,4]],"type":"G","move":{"path":[["L",1]],"loop":true}},{"body":[[11,4]],"type":"G","move":{"path":[["D",1]],"loop":true}},{"body":[[3,5]],"type":"G","move":{"path":[["R",1]],"loop":true}},{"body":[[6,5]],"type":"G","move":{"path":[["L",1]],"loop":true}},{"body":[[10,5]],"type":"G","move":{"path":[["R",1]],"loop":true}},{"body":[[12,5]],"type":"G","move":{"path":[["L",1]],"loop":true}},{"body":[[11,6]],"type":"G","move":{"path":[["U",1]],"loop":true}}]};

        this.world = new entity.World();
        this.addChild(this.world);
        this.world.seed = raws;

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
        default:
        {
            if (input.Controller.Moves.includes(event.code)) {
                if (event.code !== input.Controller.Type.MOVE_IDLE) {
                    Musician.sound(Track.CUBE_CONTROL);
                    this.order[1] = event.code;
                }
                this.order[2] = event.code;
            }
        }
    }
}

/////////////////////////////////////////////////////////////////////////////
}