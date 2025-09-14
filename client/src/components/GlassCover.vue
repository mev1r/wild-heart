<script lang="ts" setup>
import {onBeforeUnmount, onMounted, ref} from "vue";
import {Application, Container, Graphics, Sprite, Texture} from "pixi.js";
import {wait} from "../pkg/utils";

type Props = {
  width?: number;
  height?: number;
};

const {width = 40, height = 40} = defineProps<Props>();

const canvasRef = ref<HTMLCanvasElement | null>(null);
let app: Application | null = null;

function makeDiagGradientTexture(opacityPeak = 0.8) {
  const c = document.createElement("canvas");
  c.width = 512;
  c.height = 512;
  const ctx = c.getContext("2d")!;
  const g = ctx.createLinearGradient(0, 0, 512, 512);
  g.addColorStop(0.0, "rgba(255,255,255,0)");
  g.addColorStop(0.4, "rgba(255,255,255,0.0)");
  g.addColorStop(0.5, `rgba(255,255,255,${opacityPeak})`);
  g.addColorStop(0.6, "rgba(255,255,255,0.0)");
  g.addColorStop(1.0, "rgba(255,255,255,0)");
  ctx.fillStyle = g;
  ctx.fillRect(0, 0, 512, 512);
  return Texture.from(c);
}

function makeShimmerPair(tex: Texture, w: number, h: number, alpha: number) {
  const group = new Container();
  const span = Math.sqrt(w * w + h * h) * 2;
  const size = span;
  const s1 = new Sprite(tex);
  const s2 = new Sprite(tex);
  [s1, s2].forEach((s) => {
    s.width = size;
    s.height = size;
    s.alpha = alpha;
    s.anchor.set(0.5);
    s.blendMode = "screen";
    group.addChild(s);
  });

  s1.position.set(-span * 0.25, -span * 0.25);
  s2.position.set(-span * 1.25, -span * 1.25);

  const advance = (d: number) => {
    const v = 1 * d;
    s1.x += v;
    s1.y += v;
    s2.x += v;
    s2.y += v;
    const resetIfPast = (s: Sprite) => {
      if (s.x > span * 0.75 || s.y > span * 0.75) {
        s.x -= span;
        s.y -= span;
      }
    };
    resetIfPast(s1);
    resetIfPast(s2);
  };

  return {group, advance};
}

onMounted(async () => {
  if (!canvasRef.value) return;

  app = new Application();
  await app.init({
    view: canvasRef.value,
    width,
    height,
    backgroundAlpha: 0,
    antialias: true,
  });

  const W = width,
      H = height,
      BW = Math.max(1, 1);
  const ix = BW / 2,
      iy = BW / 2,
      iw = W - BW,
      ih = H - BW;

  const frame = new Graphics();
  frame
      .rect(BW + 2, BW + 2, W - 3 * BW - 3, H - 3 * BW - 3)
      .stroke({width: 1, color: 0xffffff, alpha: 0.15});
  app.stage.addChild(frame);

  const connectors = new Graphics();
  const innerX1 = BW + 2,
      innerY1 = BW + 2;
  const innerX2 = W - (BW + 2),
      innerY2 = H - (BW + 2);
  const outerX1 = ix,
      outerY1 = iy;
  const outerX2 = ix + iw,
      outerY2 = iy + ih;
  connectors
      .moveTo(innerX1, innerY1)
      .lineTo(outerX1, outerY1)
      .stroke({width: 1, color: 0xffffff, alpha: 0.15});
  connectors
      .moveTo(innerX2, innerY1)
      .lineTo(outerX2, outerY1)
      .stroke({width: 1, color: 0xffffff, alpha: 0.15});
  connectors
      .moveTo(innerX1, innerY2)
      .lineTo(outerX1, outerY2)
      .stroke({width: 1, color: 0xffffff, alpha: 0.15});
  connectors
      .moveTo(innerX2, innerY2)
      .lineTo(outerX2, outerY2)
      .stroke({width: 1, color: 0xffffff, alpha: 0.15});
  app.stage.addChild(connectors);

  const borderMask = new Graphics();
  borderMask
      .rect(ix, iy, iw, ih)
      .stroke({width: BW + 2, color: 0xffffff, alpha: 1});
  app.stage.addChild(borderMask);

  const innerMask = new Graphics();
  innerMask.rect(BW, BW, W - 2 * BW, H - 2 * BW).fill(0xffffff);
  app.stage.addChild(innerMask);

  const borderTex = makeDiagGradientTexture(0.7);
  const borderPair = makeShimmerPair(borderTex, W, H, 0.6);
  borderPair.group.mask = borderMask;
  app.stage.addChild(borderPair.group);

  const innerTex = makeDiagGradientTexture(0.35);
  const innerPair = makeShimmerPair(innerTex, W, H, 0.55);
  innerPair.group.mask = innerMask;
  app.stage.addChild(innerPair.group);

  app.ticker.add((ticker) => {
    const dt = ticker.deltaTime;
    borderPair.advance(dt);
    innerPair.advance(dt);
  });
});

onBeforeUnmount(async () => {
  if (app) {
    await wait(0.3);
    app.destroy(true, {children: true});
    app = null;
  }
});
</script>

<template>
  <canvas
      ref="canvasRef"
      class="block absolute w-full h-full pointer-events-none"
  />
</template>
