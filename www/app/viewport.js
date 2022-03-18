export class Viewport {
    constructor(el) {
        this.el = el;
    }

    clear() {
        const pixelRatio = window.devicePixelRatio || 1;

        const size = Math.min(
            window.innerWidth - 500,
            window.innerHeight - 50,
        )
        
        this.el.width = size * pixelRatio;
        this.el.height = size * pixelRatio;
        this.el.style.width = size + "px";
        this.el.style.height = size + "px";

        this.ctx = this.el.getContext("2d");
        this.ctx.clearRect(0, 0, this._size(), this._size());
    }

    drawCircle(x, y, radius, style) {
        x *= this._size();
        y *= this._size();
        radius *= this._size();

        this.ctx.beginPath();
        this.ctx.arc(x, y, radius, 0.0, 2.0 * Math.PI);
        this.ctx.fillStyle = style;
        this.ctx.fill();
    }

    drawArc(x, y, radius, angleFrom, angleTo, style) {
        x *= this._size();
        y *= this._size();
        radius *= this._size();

        this.ctx.beginPath();
        this.ctx.arc(x, y, radius, angleFrom, angleTo);
        this.ctx.strokeStyle = style;
        this.ctx.lineWidth = 0.002 * this._size();
        this.ctx.stroke();
    }

    drawTriangle(x, y, size, rotation, style) {
        x *= this._size();
        y *= this._size();
        size *= this._size();

        this.ctx.beginPath();

        this.ctx.moveTo(
            x + Math.cos(rotation) * size * 1.5,
            y + Math.sin(rotation) * size * 1.5,
        );

        this.ctx.lineTo(
            x + Math.cos(rotation + 2.0 / 3.0 * Math.PI) * size,
            y + Math.sin(rotation + 2.0 / 3.0 * Math.PI) * size,
        )

        this.ctx.lineTo(
            x + Math.cos(rotation - 2.0 / 3.0 * Math.PI) * size,
            y + Math.sin(rotation - 2.0 / 3.0 * Math.PI) * size,
        )

        this.ctx.lineTo(
            x + Math.cos(rotation) * size * 1.5,
            y + Math.sin(rotation) * size * 1.5,
        )

        this.ctx.fillStyle = style;
        this.ctx.fill();
    }

    _size() {
        return this.el.width;
    }
}
