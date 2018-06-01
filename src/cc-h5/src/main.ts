class Main extends egret.DisplayObjectContainer
{
    private input: input.KeyBoard = new input.KeyBoard();
    private order: input.Controller = input.Controller.create(this.input);
    private moves: input.Controller.Type = input.Controller.Type.MOVE_IDLE;
    private world: entity.World;
    private timer: egret.Timer = new egret.Timer(250, 0);

    public constructor()
    {
        super();
        this.addEventListener(egret.Event.ADDED_TO_STAGE, this.onAddToStage, this);
    }

    private onAddToStage(event: egret.Event)
    {
        egret.lifecycle.addLifecycleListener((context) => {
            context.onUpdate = () => {

            }
        })

        egret.lifecycle.onPause = () => {
            egret.ticker.pause();
        }

        egret.lifecycle.onResume = () => {
            egret.ticker.resume();
        }

        this.runGame().catch(e => {
            console.log(e);
        })
    }

    private async runGame()
    {
        await this.loadResource()
        this.createGameScene();
        const result = await RES.getResAsync("description_json")
    }

    private async loadResource()
    {
        try {
            const loadingView = new scene.LoadingUI();
            this.stage.addChild(loadingView);
            // await RES.loadConfig("resource/default.res.json", "resource/");
            // await RES.loadGroup("preload", 0, loadingView);
            this.stage.removeChild(loadingView);
        } catch (e) {
            console.error(e);
        }
    }

    private createGameScene()
    {
        const shape = new egret.Shape();
        this.addChild(shape);
        shape.graphics.beginFill(0x000000);
        shape.graphics.drawRect(0, 0, this.stage.stageWidth, this.stage.stageHeight);
        shape.graphics.endFill();

        const raws = <core.Seed><any>{"size":{"height":8,"width":13},"head":{"author":"MYLS","title":"Fishing"},"dest":[],"version":1,"cube":[{"body":[[0,0],[1,0],[2,0],[3,0],[4,0],[5,0],[6,0],[7,0],[8,0],[9,0],[10,0],[11,0],[12,0],[0,1],[12,1],[0,2],[8,2],[12,2],[0,3],[12,3],[0,4],[1,4],[2,4],[3,4],[4,4],[5,4],[6,4],[12,4],[0,5],[12,5],[0,6],[12,6],[0,7],[1,7],[2,7],[3,7],[4,7],[5,7],[6,7],[7,7],[8,7],[9,7],[10,7],[11,7],[12,7]],"type":"W"},{"body":[[4,1]],"type":"G"},{"body":[[2,2]],"type":"B"},{"body":[[6,2]],"type":"G"},{"body":[[9,2]],"type":"G"},{"body":[[4,3]],"type":"G"}]};

        this.world = new entity.World();
        this.addChild(this.world);
        this.world.seed = raws;

        this.order.enable = true;
        this.order.addEventListener(input.Controller.Event.ORDER, this.onControl, this);

        this.timer.start();
        this.timer.addEventListener(egret.TimerEvent.TIMER, this.onDraw, this);

        // r.graphics.beginFill(0xff0000 + Math.floor(Math.random() * 100) * (0xffffff / 100), 1);
        // r.graphics.lineStyle(2, 0xff0000 + Math.floor(Math.random() * 100) * (0xffffff / 100));
        // r.graphics.drawCircle(Math.random() * w, Math.random() * h, Math.random() * 50 + 50);
        // r.graphics.endFill();
    }

    private onDraw(event:egret.Event): void
    {
        if (this.world === undefined)
            return;

        this.world.command(this.moves as number);
        this.world.next();
    }

    private onControl(event: input.Controller.Event): void
    {
        console.log("Control", event.code);
        if (input.Controller.Moves.includes(event.code)) {
            this.moves = event.code;
        } else {
            if (event.code === input.Controller.Type.CTRL_RESTART) {
                this.world.build();
            }
        }
    }
}
