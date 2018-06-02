namespace scene {
/////////////////////////////////////////////////////////////////////////////

const Track = utils.Track;
const Musician = utils.Musician.instance;

/////////////////////////////////////////////////////////////////////////////

export class World extends eui.Component
{
    private input: input.KeyBoard = new input.KeyBoard();
    private ctrlr: input.Controller = input.Controller.create(this.input);
    private moves: input.Controller.Type = input.Controller.Type.MOVE_IDLE;
    private world: entity.World;
    private timer: egret.Timer = new egret.Timer(250, 0);

    public constructor()
    {
        super();
        this.addEventListener(egret.Event.ADDED_TO_STAGE, this.onAddToStage, this);
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

        this.ctrlr.enable = true;
        this.ctrlr.addEventListener(input.Controller.Event.ORDER, this.onControl, this);

        this.timer.start();
        this.timer.addEventListener(egret.TimerEvent.TIMER, this.onNext, this);
    }

    private onNext(event:egret.Event): void
    {
        if (this.world === undefined)
            return;

        this.world.command(this.moves as number);
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
                if (seed !== undefined)
                    this.world.build(seed);
            }
            break;
        }
        default:
        {
            if (input.Controller.Moves.includes(event.code)) {
                if (event.code !== input.Controller.Type.MOVE_IDLE) {
                    //////// Music ////////
                    Musician.sound(Track.CUBE_CONTROL);
                    this.press = true;
                }
                this.order = event.code;
            }
        }
    }

}

/////////////////////////////////////////////////////////////////////////////
}