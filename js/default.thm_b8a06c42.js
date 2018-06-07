window.layout={};
                function __extends(d, b) {
                    for (var p in b) if (b.hasOwnProperty(p)) d[p] = b[p];
                        function __() {
                            this.constructor = d;
                        }
                    __.prototype = b.prototype;
                    d.prototype = new __();
                };
                window.generateEUI = {};
                generateEUI.paths = {};
                generateEUI.styles = undefined;
                generateEUI.skins = {"layout.Loading":"resource/layout/layout_loading.exml"};generateEUI.paths['resource/layout/layout_loading.exml'] = window.layout.Loading = (function (_super) {
	__extends(Loading, _super);
	function Loading() {
		_super.call(this);
		this.skinParts = ["rect_background"];
		
		this.height = 720;
		this.width = 1280;
		this.elementsContent = [this.rect_background_i()];
	}
	var _proto = Loading.prototype;

	_proto.rect_background_i = function () {
		var t = new eui.Rect();
		this.rect_background = t;
		t.anchorOffsetX = 0;
		t.anchorOffsetY = 0;
		t.fillColor = 0xffffff;
		t.percentHeight = 100;
		t.left = 0;
		t.verticalCenter = 0;
		t.width = 0;
		return t;
	};
	return Loading;
})(eui.Skin);