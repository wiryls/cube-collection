class LoadingUI extends eui.Group implements RES.PromiseTaskReporter
{
    private text: egret.TextField;
    private counter: number = 0;
    private _txProgress: egret.TextField;
    private _loadingRun: egret.Bitmap;
    public constructor(public groups: ReadonlyArray<string>)
    {
        super();
    }

    protected createChildren(): void
    {
        super.createChildren();

        this.image_background = new egret.Bitmap(RES.getRes("ui_image_background"));
        this.addChild(this.image_background);
        this.image_background.width  = this.stage.stageWidth;
        this.image_background.height = this.stage.stageHeight;

        // this.image_circle = new egret.Bitmap(RES.getRes("ui_image_loading"));
        // this.addChild(this.image_circle);
        // this.image_circle.anchorOffsetX = this.image_circle.width * .5;
        // this.image_circle.anchorOffsetY = this.image_circle.height * .5;
        // this.image_circle.x = this.stage.stageWidth * .5;
        // this.image_circle.y = this.stage.stageHeight * .5;
        // egret.Tween
        //     .get(this.image_circle,{loop:true})
        //     .to({rotation: 360}, 1000)
        //     ;

        this.text_progress = new egret.TextField();
        this.addChild(this.text_progress);
        this.text_progress.textColor = 0xAAAAAA;
        this.text_progress.x = this.stage.stageHeight * .5;
        this.text_progress.y = this.stage.stageHeight * .5;
        this.text_progress.width = 480;
        this.text_progress.height = 100;
        this.text_progress.textAlign = "center";
    }

    public count(): void
    {
        if (this.counter < this.groups.length)
            this.counter += 1;
    }

    public onProgress(current: number, total: number): void
    {
        const group = this.groups.length;
        const index = this.counter;
        const scale = 100.0 / group;
        const percent = Math.round(scale * index + scale * current / total);

        this.text_progress.text = `Loading ${percent}%`;
    }
}
