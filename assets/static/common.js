const atomStyle = {
    salt: { shadowStyle: 'black', fillStyle: '#eee', symbolStyle: '#ccc', symbol: [
        { type: 'arc', x: 0, y: 0, radius: 0.5, from: 0, to: 1 },
        { type: 'line', points: [ -0.5, 0, 0.5, 0 ] },
    ] },
    air: { shadowStyle: 'black', fillStyle: '#bef', symbolStyle: '#9cc', symbol: [
        { type: 'loop', points: [ 0, -0.45, 0.45, 0.32, -0.45, 0.32 ] },
        { type: 'line', points: [ -0.5, -0.05, 0.5, -0.05 ] },
    ] },
    fire: { shadowStyle: 'black', fillStyle: '#f45', symbolStyle: '#c34', symbol: [
        { type: 'loop', points: [ 0, -0.45, 0.45, 0.32, -0.45, 0.32 ] },
    ] },
    quicksilver: { shadowStyle: 'black', fillStyle: '#ddd', symbolStyle: '#bbb', symbol: [
        { type: 'arc', x: 0, y: -0.6, radius: 0.25, from: 0, to: 0.5 },
        { type: 'arc', x: 0, y: -0.05, radius: 0.3, from: 0, to: 1 },
        { type: 'line', points: [ 0, 0.25, 0, 0.65 ] },
        { type: 'line', points: [ -0.2, 0.45, 0.2, 0.45 ] },
    ] },
    water: { shadowStyle: 'black', fillStyle: '#0bf', symbolStyle: '#09c', symbol: [
        { type: 'loop', points: [ 0, 0.45, 0.45, -0.32, -0.45, -0.32 ] },
    ] },
    earth: { shadowStyle: 'black', fillStyle: '#6e4', symbolStyle: '#4c2', symbol: [
        { type: 'loop', points: [ 0, 0.45, 0.45, -0.32, -0.45, -0.32 ] },
        { type: 'line', points: [ -0.5, 0, 0.5, 0 ] },
    ] },
    lead: { shadowStyle: 'black', fillStyle: '#458', symbolStyle: '#67b', symbol: [
        { type: 'arc', x: 0.075, y: 0.2, radius: 0.3, from: 0.5, to: 1.3 },
        { type: 'line', points: [ -0.225, -0.5, -0.225, 0.2 ] },
        { type: 'line', points: [ -0.425, -0.3, -0.025, -0.3 ] },
    ] },
    tin: { shadowStyle: 'black', fillStyle: '#876', symbolStyle: '#a98', symbol: [
        { type: 'line', points: [ -0.4, 0.2, 0.55, 0.2 ] },
        { type: 'line', points: [ 0.25, -0.4, 0.25, 0.5 ] },
        { type: 'arc', x: -0.2, y: -0.05, radius: 0.25, from: 0.55, to: 1.25 },
    ] },
    iron: { shadowStyle: 'black', fillStyle: '#844', symbolStyle: '#b66', symbol: [
        { type: 'arc', x: -0.15, y: 0.15, radius: 0.3, from: 0, to: 1 },
        { type: 'line', points: [ 0.062, -0.062, 0.4, -0.4 ] },
        { type: 'line', points: [ 0, -0.4, 0.4, -0.4, 0.4, 0 ] },
    ] },
    copper: { shadowStyle: 'black', fillStyle: '#b74', symbolStyle: '#d96', symbol: [
        { type: 'arc', x: 0, y: -0.2, radius: 0.3, from: 0, to: 1 },
        { type: 'line', points: [ 0, 0.1, 0, 0.55 ] },
        { type: 'line', points: [ -0.2, 0.325, 0.2, 0.325 ] },
    ] },
    silver: { shadowStyle: 'black', fillStyle: '#334', symbolStyle: '#556', symbol: [
        { type: 'arc', x: 0, y: 0, radius: 0.5, from: 0.6, to: 0.4 },
        { type: 'arc', x: -0.6, y: 0, radius: 0.8, from: 0.9, to: 0.1 },
        // { type: 'line', points: [ 0.15, -0.5, 0.15, 0.5 ] },
    ] },
    gold: { shadowStyle: 'black', fillStyle: '#d92', symbolStyle: '#fb3', symbol: [
        { type: 'arc', x: 0, y: 0, radius: 0.5, from: 0, to: 1 },
        { type: 'arc', x: 0, y: 0, radius: 0.05, from: 0, to: 1 },
    ] },
    vitae: { shadowStyle: 'black', fillStyle: '#fcc', symbolStyle: '#d99', symbol: [
        { type: 'loop', points: [ 0, -0.45, 0.32, 0.1, -0.32, 0.1 ] },
        { type: 'line', points: [ 0, 0.1, 0, 0.55 ] },
        { type: 'line', points: [ -0.2, 0.325, 0.2, 0.325 ] },
    ] },
    // mors: { shadowStyle: 'black', fillStyle: '#444', symbolStyle: '#222', symbol: [
    mors: { shadowStyle: 'black', fillStyle: '#444', symbolStyle: '#666', symbol: [
        { type: 'loop', points: [ 0, 0.45, 0.32, -0.1, -0.32, -0.1 ] },
        { type: 'line', points: [ 0, -0.1, 0, -0.55 ] },
        { type: 'line', points: [ -0.2, -0.325, 0.2, -0.325 ] },
    ] },
    quintessence: { shadowStyle: 'black', fillStyle: '#546', symbolStyle: '#768', symbol: [
        { type: 'loop', points: [ 0, 0.5, 0.45, -0.27, -0.45, -0.27 ] },
        { type: 'line', points: [ -0.15, -0.27, 0, -0.5, 0.15, -0.27 ] },
        { type: 'line', points: [ 0.3, 0, 0.45, 0.27, 0.15, 0.27 ] },
        { type: 'line', points: [ -0.3, 0, -0.45, 0.27, -0.15, 0.27 ] },
    ] },
    repeat: { shadowStyle: 'black', fillStyle: '#555', symbolStyle: '#333', symbol: [
        { type: 'arc', x: 0, y: 0, radius: 0.05, from: 0, to: 1 },
        { type: 'arc', x: -0.4, y: 0, radius: 0.05, from: 0, to: 1 },
        { type: 'arc', x: 0.4, y: 0, radius: 0.05, from: 0, to: 1 },
    ] },
};

function drawAtom(ctx, atom, x, y) {
    ctx.fillStyle = atomStyle[atom].fillStyle;
    ctx.beginPath();
    ctx.ellipse(x, y, 29, 29, 0, 0, 2 * Math.PI);
    ctx.fill();
    ctx.save();
    ctx.translate(x, y);
    ctx.scale(29, 29);
    ctx.lineWidth = 3 / 29;
    ctx.strokeStyle = atomStyle[atom].symbolStyle;
    for (const s of atomStyle[atom].symbol) {
        if (s.type === 'arc') {
            ctx.beginPath();
            ctx.ellipse(s.x, s.y, s.radius, s.radius, 0, s.from * 2 * Math.PI, s.to * 2 * Math.PI);
            ctx.stroke();
        } else if (s.type == 'line') {
            ctx.beginPath();
            ctx.moveTo(s.points[0], s.points[1]);
            for (let i = 2; i < s.points.length; i += 2)
                ctx.lineTo(s.points[i], s.points[i + 1]);
            ctx.stroke();
        } else if (s.type == 'loop') {
            ctx.beginPath();
            ctx.moveTo(s.points[0], s.points[1]);
            for (let i = 2; i < s.points.length; i += 2)
                ctx.lineTo(s.points[i], s.points[i + 1]);
            ctx.closePath();
            ctx.stroke();
        }
    }
    ctx.restore();
}

function drawBond(ctx, red, black, yellow, length, shadow) {
    if (red || black || yellow) {
        if (yellow) {
            ctx.lineWidth = 3;
            ctx.strokeStyle = shadow === 'shadow' ? '#0008' : '#fc5';
            ctx.beginPath();
            ctx.moveTo(-length / 2, -5.5);
            ctx.bezierCurveTo(length / 3, -5.5, -length / 3, 5.5, length / 2, 5.5);
            ctx.stroke();
        }
        if (black) {
            ctx.lineWidth = 3;
            ctx.strokeStyle = shadow === 'shadow' ? '#0008' : '#aaa';
            ctx.beginPath();
            ctx.moveTo(-length / 2, 0);
            ctx.lineTo(length / 2, 0);
            ctx.stroke();
        }
        if (red) {
            ctx.lineWidth = 3;
            ctx.strokeStyle = shadow === 'shadow' ? '#0008' : '#f45';
            ctx.beginPath();
            ctx.moveTo(-length / 2, 5.5);
            ctx.bezierCurveTo(length / 3, 5.5, -length / 3, -5.5, length / 2, -5.5);
            ctx.stroke();
        }
    } else {
        ctx.fillStyle = shadow === 'shadow' ? '#0008' : 'white';
        ctx.fillRect(-length / 2, -5, length, 10);
    }
}

function drawProductAtom(pctx, atom, min_x, i, j, shadow) {
    const x = 45 + 82 * (i + 0.5 * j - 0.5 * min_x);
    const y = 45 + 71 * j;
    if (shadow) {
        pctx.fillStyle = atomStyle[atom].shadowStyle;
        pctx.beginPath();
        pctx.ellipse(x + shadow, y + shadow, 29, 29, 0, 0, 2 * Math.PI);
        pctx.fill();
        return;
    }
    drawAtom(pctx, atom, x, y);
}

function drawProductBond(pctx, red, black, yellow, min_x, i, j, rotation, shadow) {
    const x = 45 + 82 * (i + 0.5 * j - 0.5 * min_x);
    const y = 45 + 71 * j;
    pctx.save();
    pctx.translate(x, y);
    if (shadow)
        pctx.translate(4, 4);
    else
        pctx.translate(2, 2);
    pctx.rotate(rotation * 2 * Math.PI);
    pctx.translate(41, 0);
    if (shadow)
        drawBond(pctx, red, black, yellow, 82, 'shadow');
    else
        drawBond(pctx, red, black, yellow, 82);
    pctx.restore();
}
