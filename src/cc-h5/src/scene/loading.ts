namespace scene {
/////////////////////////////////////////////////////////////////////////////

export class Loading extends eui.Component implements RES.PromiseTaskReporter
{
    private rect_background: eui.Rect;

    private groups: ReadonlyArray<string>;
    private progress: number = 0;
    private text_progress: egret.TextField = new egret.TextField();

    public constructor()
    {
        super();

        this.addEventListener(egret.Event.ADDED_TO_STAGE, this.onAddToStage, this);
        this.addEventListener(eui.UIEvent.COMPLETE, this.onUIComplete, this);
        this.skinName = "layout.Loading";
    }

    private onUIComplete(): void
    {
        ;
    }

    private onAddToStage(): void
    {
        this.width  = this.stage.stageWidth;
        this.height = this.stage.stageHeight;

        Musician.music(utils.Track.BGM_NORMAL);
    }

    protected createChildren(): void
    {
        super.createChildren();

        this.addChild(this.text_progress);
        this.text_progress.textColor = 0x777777;
        this.text_progress.x = this.stage.stageHeight * .5;
        this.text_progress.y = this.stage.stageHeight * .5;
        this.text_progress.width = 480;
        this.text_progress.height = 100;
        this.text_progress.textAlign = "center";
    }

    public set track(groups: ReadonlyArray<string>)
    {
        this.progress = 0;
        this.groups = groups;
    }

    public count(delta: number = 1): void
    {
        this.progress += delta;
        if (this.progress >= this.groups.length) {
            this.progress = this.groups.length - 1;
            this.text_progress.visible = false;
        } else {
            this.text_progress.visible = true;
        }
    }

    public onProgress(current: number, total: number): void
    {
        const group = this.groups.length;
        const index = this.progress;
        const scale = 100.0 / group;
        const percent = Math.round(scale * index + scale * current / total);

        this.rect_background.width = this.width * percent / 100;
        this.text_progress.text = `Loading ${percent}%`;
    }
}

/////////////////////////////////////////////////////////////////////////////
}