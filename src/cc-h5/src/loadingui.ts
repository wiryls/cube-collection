class LoadingUI extends egret.Sprite implements RES.PromiseTaskReporter
{
    private text: egret.TextField;
    private counter: number = 0;

    public constructor(public groups: ReadonlyArray<string>)
    {
        super();
        this.onCreateView();
    }

    private onCreateView(): void
    {
        this.text = new egret.TextField();
        this.addChild(this.text);
        this.text.y = 300;
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
