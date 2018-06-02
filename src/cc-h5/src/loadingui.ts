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
        this._loadingRun = new egret.Bitmap(RES.getRes("loadingrun_png"));
        this.addChild( this._loadingRun );
        this._loadingRun.anchorOffsetX = this._loadingRun.width * .5;
        this._loadingRun.anchorOffsetY = this._loadingRun.height * .5;
        this._loadingRun.x = this.stage.stageWidth * .5;
        this._loadingRun.y = this.stage.stageHeight * .5;
        egret.Tween
            .get(this._loadingRun,{loop:true})
            .to({rotation: 360}, 1000)
            ;

        this.text = new egret.TextField();
        this.addChild(this.text);
        this.text.x = this.stage.stageHeight * .5;        
        this.text.y = this.stage.stageHeight * .5 -100;
        this.text.width = 480;
        this.text.height = 100;
        this.text.textAlign = "center";
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

        this.text.text = `Loading...${percent}%`;
    }
}
