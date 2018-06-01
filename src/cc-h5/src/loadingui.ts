namespace scene {
/////////////////////////////////////////////////////////////////////////////

export class LoadingUI extends egret.Sprite implements RES.PromiseTaskReporter
{
    private textField: egret.TextField;

    public constructor()
    {
        super();
        this.onCreate();
    }

    private onCreate(): void
    {
        this.textField = new egret.TextField();
        this.addChild(this.textField);
        this.textField.y = 300;
        this.textField.width = 480;
        this.textField.height = 100;
        this.textField.textAlign = "center";
    }

    public onProgress(current: number, total: number): void
    {
        this.textField.text = `Loading...${current}/${total}`;
    }
}

/////////////////////////////////////////////////////////////////////////////
}