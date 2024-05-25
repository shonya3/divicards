var ci = (e, t, i) => {
  if (!t.has(e))
    throw TypeError("Cannot " + i);
};
var te = (e, t, i) => (ci(e, t, "read from private field"), i ? i.call(e) : t.get(e)), ee = (e, t, i) => {
  if (t.has(e))
    throw TypeError("Cannot add the same private member more than once");
  t instanceof WeakSet ? t.add(e) : t.set(e, i);
};
/**
 * @license
 * Copyright 2019 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const mt = globalThis, Vt = mt.ShadowRoot && (mt.ShadyCSS === void 0 || mt.ShadyCSS.nativeShadow) && "adoptedStyleSheets" in Document.prototype && "replace" in CSSStyleSheet.prototype, qt = Symbol(), ie = /* @__PURE__ */ new WeakMap();
let Ae = class {
  constructor(t, i, s) {
    if (this._$cssResult$ = !0, s !== qt)
      throw Error("CSSResult is not constructable. Use `unsafeCSS` or `css` instead.");
    this.cssText = t, this.t = i;
  }
  get styleSheet() {
    let t = this.o;
    const i = this.t;
    if (Vt && t === void 0) {
      const s = i !== void 0 && i.length === 1;
      s && (t = ie.get(i)), t === void 0 && ((this.o = t = new CSSStyleSheet()).replaceSync(this.cssText), s && ie.set(i, t));
    }
    return t;
  }
  toString() {
    return this.cssText;
  }
};
const hi = (e) => new Ae(typeof e == "string" ? e : e + "", void 0, qt), S = (e, ...t) => {
  const i = e.length === 1 ? e[0] : t.reduce((s, n, r) => s + ((o) => {
    if (o._$cssResult$ === !0)
      return o.cssText;
    if (typeof o == "number")
      return o;
    throw Error("Value passed to 'css' function must be a 'css' function result: " + o + ". Use 'unsafeCSS' to pass non-literal values, but take care to ensure page security.");
  })(n) + e[r + 1], e[0]);
  return new Ae(i, e, qt);
}, di = (e, t) => {
  if (Vt)
    e.adoptedStyleSheets = t.map((i) => i instanceof CSSStyleSheet ? i : i.styleSheet);
  else
    for (const i of t) {
      const s = document.createElement("style"), n = mt.litNonce;
      n !== void 0 && s.setAttribute("nonce", n), s.textContent = i.cssText, e.appendChild(s);
    }
}, se = Vt ? (e) => e : (e) => e instanceof CSSStyleSheet ? ((t) => {
  let i = "";
  for (const s of t.cssRules)
    i += s.cssText;
  return hi(i);
})(e) : e;
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const { is: ui, defineProperty: pi, getOwnPropertyDescriptor: fi, getOwnPropertyNames: mi, getOwnPropertySymbols: gi, getPrototypeOf: yi } = Object, j = globalThis, ne = j.trustedTypes, vi = ne ? ne.emptyScript : "", jt = j.reactiveElementPolyfillSupport, it = (e, t) => e, gt = { toAttribute(e, t) {
  switch (t) {
    case Boolean:
      e = e ? vi : null;
      break;
    case Object:
    case Array:
      e = e == null ? e : JSON.stringify(e);
  }
  return e;
}, fromAttribute(e, t) {
  let i = e;
  switch (t) {
    case Boolean:
      i = e !== null;
      break;
    case Number:
      i = e === null ? null : Number(e);
      break;
    case Object:
    case Array:
      try {
        i = JSON.parse(e);
      } catch {
        i = null;
      }
  }
  return i;
} }, Ft = (e, t) => !ui(e, t), re = { attribute: !0, type: String, converter: gt, reflect: !1, hasChanged: Ft };
Symbol.metadata ?? (Symbol.metadata = Symbol("metadata")), j.litPropertyMetadata ?? (j.litPropertyMetadata = /* @__PURE__ */ new WeakMap());
class W extends HTMLElement {
  static addInitializer(t) {
    this._$Ei(), (this.l ?? (this.l = [])).push(t);
  }
  static get observedAttributes() {
    return this.finalize(), this._$Eh && [...this._$Eh.keys()];
  }
  static createProperty(t, i = re) {
    if (i.state && (i.attribute = !1), this._$Ei(), this.elementProperties.set(t, i), !i.noAccessor) {
      const s = Symbol(), n = this.getPropertyDescriptor(t, s, i);
      n !== void 0 && pi(this.prototype, t, n);
    }
  }
  static getPropertyDescriptor(t, i, s) {
    const { get: n, set: r } = fi(this.prototype, t) ?? { get() {
      return this[i];
    }, set(o) {
      this[i] = o;
    } };
    return { get() {
      return n == null ? void 0 : n.call(this);
    }, set(o) {
      const a = n == null ? void 0 : n.call(this);
      r.call(this, o), this.requestUpdate(t, a, s);
    }, configurable: !0, enumerable: !0 };
  }
  static getPropertyOptions(t) {
    return this.elementProperties.get(t) ?? re;
  }
  static _$Ei() {
    if (this.hasOwnProperty(it("elementProperties")))
      return;
    const t = yi(this);
    t.finalize(), t.l !== void 0 && (this.l = [...t.l]), this.elementProperties = new Map(t.elementProperties);
  }
  static finalize() {
    if (this.hasOwnProperty(it("finalized")))
      return;
    if (this.finalized = !0, this._$Ei(), this.hasOwnProperty(it("properties"))) {
      const i = this.properties, s = [...mi(i), ...gi(i)];
      for (const n of s)
        this.createProperty(n, i[n]);
    }
    const t = this[Symbol.metadata];
    if (t !== null) {
      const i = litPropertyMetadata.get(t);
      if (i !== void 0)
        for (const [s, n] of i)
          this.elementProperties.set(s, n);
    }
    this._$Eh = /* @__PURE__ */ new Map();
    for (const [i, s] of this.elementProperties) {
      const n = this._$Eu(i, s);
      n !== void 0 && this._$Eh.set(n, i);
    }
    this.elementStyles = this.finalizeStyles(this.styles);
  }
  static finalizeStyles(t) {
    const i = [];
    if (Array.isArray(t)) {
      const s = new Set(t.flat(1 / 0).reverse());
      for (const n of s)
        i.unshift(se(n));
    } else
      t !== void 0 && i.push(se(t));
    return i;
  }
  static _$Eu(t, i) {
    const s = i.attribute;
    return s === !1 ? void 0 : typeof s == "string" ? s : typeof t == "string" ? t.toLowerCase() : void 0;
  }
  constructor() {
    super(), this._$Ep = void 0, this.isUpdatePending = !1, this.hasUpdated = !1, this._$Em = null, this._$Ev();
  }
  _$Ev() {
    var t;
    this._$ES = new Promise((i) => this.enableUpdating = i), this._$AL = /* @__PURE__ */ new Map(), this._$E_(), this.requestUpdate(), (t = this.constructor.l) == null || t.forEach((i) => i(this));
  }
  addController(t) {
    var i;
    (this._$EO ?? (this._$EO = /* @__PURE__ */ new Set())).add(t), this.renderRoot !== void 0 && this.isConnected && ((i = t.hostConnected) == null || i.call(t));
  }
  removeController(t) {
    var i;
    (i = this._$EO) == null || i.delete(t);
  }
  _$E_() {
    const t = /* @__PURE__ */ new Map(), i = this.constructor.elementProperties;
    for (const s of i.keys())
      this.hasOwnProperty(s) && (t.set(s, this[s]), delete this[s]);
    t.size > 0 && (this._$Ep = t);
  }
  createRenderRoot() {
    const t = this.shadowRoot ?? this.attachShadow(this.constructor.shadowRootOptions);
    return di(t, this.constructor.elementStyles), t;
  }
  connectedCallback() {
    var t;
    this.renderRoot ?? (this.renderRoot = this.createRenderRoot()), this.enableUpdating(!0), (t = this._$EO) == null || t.forEach((i) => {
      var s;
      return (s = i.hostConnected) == null ? void 0 : s.call(i);
    });
  }
  enableUpdating(t) {
  }
  disconnectedCallback() {
    var t;
    (t = this._$EO) == null || t.forEach((i) => {
      var s;
      return (s = i.hostDisconnected) == null ? void 0 : s.call(i);
    });
  }
  attributeChangedCallback(t, i, s) {
    this._$AK(t, s);
  }
  _$EC(t, i) {
    var r;
    const s = this.constructor.elementProperties.get(t), n = this.constructor._$Eu(t, s);
    if (n !== void 0 && s.reflect === !0) {
      const o = (((r = s.converter) == null ? void 0 : r.toAttribute) !== void 0 ? s.converter : gt).toAttribute(i, s.type);
      this._$Em = t, o == null ? this.removeAttribute(n) : this.setAttribute(n, o), this._$Em = null;
    }
  }
  _$AK(t, i) {
    var r;
    const s = this.constructor, n = s._$Eh.get(t);
    if (n !== void 0 && this._$Em !== n) {
      const o = s.getPropertyOptions(n), a = typeof o.converter == "function" ? { fromAttribute: o.converter } : ((r = o.converter) == null ? void 0 : r.fromAttribute) !== void 0 ? o.converter : gt;
      this._$Em = n, this[n] = a.fromAttribute(i, o.type), this._$Em = null;
    }
  }
  requestUpdate(t, i, s) {
    if (t !== void 0) {
      if (s ?? (s = this.constructor.getPropertyOptions(t)), !(s.hasChanged ?? Ft)(this[t], i))
        return;
      this.P(t, i, s);
    }
    this.isUpdatePending === !1 && (this._$ES = this._$ET());
  }
  P(t, i, s) {
    this._$AL.has(t) || this._$AL.set(t, i), s.reflect === !0 && this._$Em !== t && (this._$Ej ?? (this._$Ej = /* @__PURE__ */ new Set())).add(t);
  }
  async _$ET() {
    this.isUpdatePending = !0;
    try {
      await this._$ES;
    } catch (i) {
      Promise.reject(i);
    }
    const t = this.scheduleUpdate();
    return t != null && await t, !this.isUpdatePending;
  }
  scheduleUpdate() {
    return this.performUpdate();
  }
  performUpdate() {
    var s;
    if (!this.isUpdatePending)
      return;
    if (!this.hasUpdated) {
      if (this.renderRoot ?? (this.renderRoot = this.createRenderRoot()), this._$Ep) {
        for (const [r, o] of this._$Ep)
          this[r] = o;
        this._$Ep = void 0;
      }
      const n = this.constructor.elementProperties;
      if (n.size > 0)
        for (const [r, o] of n)
          o.wrapped !== !0 || this._$AL.has(r) || this[r] === void 0 || this.P(r, this[r], o);
    }
    let t = !1;
    const i = this._$AL;
    try {
      t = this.shouldUpdate(i), t ? (this.willUpdate(i), (s = this._$EO) == null || s.forEach((n) => {
        var r;
        return (r = n.hostUpdate) == null ? void 0 : r.call(n);
      }), this.update(i)) : this._$EU();
    } catch (n) {
      throw t = !1, this._$EU(), n;
    }
    t && this._$AE(i);
  }
  willUpdate(t) {
  }
  _$AE(t) {
    var i;
    (i = this._$EO) == null || i.forEach((s) => {
      var n;
      return (n = s.hostUpdated) == null ? void 0 : n.call(s);
    }), this.hasUpdated || (this.hasUpdated = !0, this.firstUpdated(t)), this.updated(t);
  }
  _$EU() {
    this._$AL = /* @__PURE__ */ new Map(), this.isUpdatePending = !1;
  }
  get updateComplete() {
    return this.getUpdateComplete();
  }
  getUpdateComplete() {
    return this._$ES;
  }
  shouldUpdate(t) {
    return !0;
  }
  update(t) {
    this._$Ej && (this._$Ej = this._$Ej.forEach((i) => this._$EC(i, this[i]))), this._$EU();
  }
  updated(t) {
  }
  firstUpdated(t) {
  }
}
W.elementStyles = [], W.shadowRootOptions = { mode: "open" }, W[it("elementProperties")] = /* @__PURE__ */ new Map(), W[it("finalized")] = /* @__PURE__ */ new Map(), jt == null || jt({ ReactiveElement: W }), (j.reactiveElementVersions ?? (j.reactiveElementVersions = [])).push("2.0.4");
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const st = globalThis, yt = st.trustedTypes, oe = yt ? yt.createPolicy("lit-html", { createHTML: (e) => e }) : void 0, Se = "$lit$", L = `lit$${Math.random().toFixed(9).slice(2)}$`, Ee = "?" + L, wi = `<${Ee}>`, q = document, nt = () => q.createComment(""), rt = (e) => e === null || typeof e != "object" && typeof e != "function", ke = Array.isArray, bi = (e) => ke(e) || typeof (e == null ? void 0 : e[Symbol.iterator]) == "function", zt = `[ 	
\f\r]`, tt = /<(?:(!--|\/[^a-zA-Z])|(\/?[a-zA-Z][^>\s]*)|(\/?$))/g, le = /-->/g, ae = />/g, B = RegExp(`>|${zt}(?:([^\\s"'>=/]+)(${zt}*=${zt}*(?:[^ 	
\f\r"'\`<>=]|("|')|))|$)`, "g"), ce = /'/g, he = /"/g, Pe = /^(?:script|style|textarea|title)$/i, $i = (e) => (t, ...i) => ({ _$litType$: e, strings: t, values: i }), u = $i(1), D = Symbol.for("lit-noChange"), f = Symbol.for("lit-nothing"), de = /* @__PURE__ */ new WeakMap(), V = q.createTreeWalker(q, 129);
function Oe(e, t) {
  if (!Array.isArray(e) || !e.hasOwnProperty("raw"))
    throw Error("invalid template strings array");
  return oe !== void 0 ? oe.createHTML(t) : t;
}
const xi = (e, t) => {
  const i = e.length - 1, s = [];
  let n, r = t === 2 ? "<svg>" : "", o = tt;
  for (let a = 0; a < i; a++) {
    const l = e[a];
    let c, d, h = -1, m = 0;
    for (; m < l.length && (o.lastIndex = m, d = o.exec(l), d !== null); )
      m = o.lastIndex, o === tt ? d[1] === "!--" ? o = le : d[1] !== void 0 ? o = ae : d[2] !== void 0 ? (Pe.test(d[2]) && (n = RegExp("</" + d[2], "g")), o = B) : d[3] !== void 0 && (o = B) : o === B ? d[0] === ">" ? (o = n ?? tt, h = -1) : d[1] === void 0 ? h = -2 : (h = o.lastIndex - d[2].length, c = d[1], o = d[3] === void 0 ? B : d[3] === '"' ? he : ce) : o === he || o === ce ? o = B : o === le || o === ae ? o = tt : (o = B, n = void 0);
    const p = o === B && e[a + 1].startsWith("/>") ? " " : "";
    r += o === tt ? l + wi : h >= 0 ? (s.push(c), l.slice(0, h) + Se + l.slice(h) + L + p) : l + L + (h === -2 ? a : p);
  }
  return [Oe(e, r + (e[i] || "<?>") + (t === 2 ? "</svg>" : "")), s];
};
class ot {
  constructor({ strings: t, _$litType$: i }, s) {
    let n;
    this.parts = [];
    let r = 0, o = 0;
    const a = t.length - 1, l = this.parts, [c, d] = xi(t, i);
    if (this.el = ot.createElement(c, s), V.currentNode = this.el.content, i === 2) {
      const h = this.el.content.firstChild;
      h.replaceWith(...h.childNodes);
    }
    for (; (n = V.nextNode()) !== null && l.length < a; ) {
      if (n.nodeType === 1) {
        if (n.hasAttributes())
          for (const h of n.getAttributeNames())
            if (h.endsWith(Se)) {
              const m = d[o++], p = n.getAttribute(h).split(L), g = /([.?@])?(.*)/.exec(m);
              l.push({ type: 1, index: r, name: g[2], strings: p, ctor: g[1] === "." ? Ai : g[1] === "?" ? Si : g[1] === "@" ? Ei : Pt }), n.removeAttribute(h);
            } else
              h.startsWith(L) && (l.push({ type: 6, index: r }), n.removeAttribute(h));
        if (Pe.test(n.tagName)) {
          const h = n.textContent.split(L), m = h.length - 1;
          if (m > 0) {
            n.textContent = yt ? yt.emptyScript : "";
            for (let p = 0; p < m; p++)
              n.append(h[p], nt()), V.nextNode(), l.push({ type: 2, index: ++r });
            n.append(h[m], nt());
          }
        }
      } else if (n.nodeType === 8)
        if (n.data === Ee)
          l.push({ type: 2, index: r });
        else {
          let h = -1;
          for (; (h = n.data.indexOf(L, h + 1)) !== -1; )
            l.push({ type: 7, index: r }), h += L.length - 1;
        }
      r++;
    }
  }
  static createElement(t, i) {
    const s = q.createElement("template");
    return s.innerHTML = t, s;
  }
}
function J(e, t, i = e, s) {
  var o, a;
  if (t === D)
    return t;
  let n = s !== void 0 ? (o = i._$Co) == null ? void 0 : o[s] : i._$Cl;
  const r = rt(t) ? void 0 : t._$litDirective$;
  return (n == null ? void 0 : n.constructor) !== r && ((a = n == null ? void 0 : n._$AO) == null || a.call(n, !1), r === void 0 ? n = void 0 : (n = new r(e), n._$AT(e, i, s)), s !== void 0 ? (i._$Co ?? (i._$Co = []))[s] = n : i._$Cl = n), n !== void 0 && (t = J(e, n._$AS(e, t.values), n, s)), t;
}
class _i {
  constructor(t, i) {
    this._$AV = [], this._$AN = void 0, this._$AD = t, this._$AM = i;
  }
  get parentNode() {
    return this._$AM.parentNode;
  }
  get _$AU() {
    return this._$AM._$AU;
  }
  u(t) {
    const { el: { content: i }, parts: s } = this._$AD, n = ((t == null ? void 0 : t.creationScope) ?? q).importNode(i, !0);
    V.currentNode = n;
    let r = V.nextNode(), o = 0, a = 0, l = s[0];
    for (; l !== void 0; ) {
      if (o === l.index) {
        let c;
        l.type === 2 ? c = new dt(r, r.nextSibling, this, t) : l.type === 1 ? c = new l.ctor(r, l.name, l.strings, this, t) : l.type === 6 && (c = new ki(r, this, t)), this._$AV.push(c), l = s[++a];
      }
      o !== (l == null ? void 0 : l.index) && (r = V.nextNode(), o++);
    }
    return V.currentNode = q, n;
  }
  p(t) {
    let i = 0;
    for (const s of this._$AV)
      s !== void 0 && (s.strings !== void 0 ? (s._$AI(t, s, i), i += s.strings.length - 2) : s._$AI(t[i])), i++;
  }
}
class dt {
  get _$AU() {
    var t;
    return ((t = this._$AM) == null ? void 0 : t._$AU) ?? this._$Cv;
  }
  constructor(t, i, s, n) {
    this.type = 2, this._$AH = f, this._$AN = void 0, this._$AA = t, this._$AB = i, this._$AM = s, this.options = n, this._$Cv = (n == null ? void 0 : n.isConnected) ?? !0;
  }
  get parentNode() {
    let t = this._$AA.parentNode;
    const i = this._$AM;
    return i !== void 0 && (t == null ? void 0 : t.nodeType) === 11 && (t = i.parentNode), t;
  }
  get startNode() {
    return this._$AA;
  }
  get endNode() {
    return this._$AB;
  }
  _$AI(t, i = this) {
    t = J(this, t, i), rt(t) ? t === f || t == null || t === "" ? (this._$AH !== f && this._$AR(), this._$AH = f) : t !== this._$AH && t !== D && this._(t) : t._$litType$ !== void 0 ? this.$(t) : t.nodeType !== void 0 ? this.T(t) : bi(t) ? this.k(t) : this._(t);
  }
  S(t) {
    return this._$AA.parentNode.insertBefore(t, this._$AB);
  }
  T(t) {
    this._$AH !== t && (this._$AR(), this._$AH = this.S(t));
  }
  _(t) {
    this._$AH !== f && rt(this._$AH) ? this._$AA.nextSibling.data = t : this.T(q.createTextNode(t)), this._$AH = t;
  }
  $(t) {
    var r;
    const { values: i, _$litType$: s } = t, n = typeof s == "number" ? this._$AC(t) : (s.el === void 0 && (s.el = ot.createElement(Oe(s.h, s.h[0]), this.options)), s);
    if (((r = this._$AH) == null ? void 0 : r._$AD) === n)
      this._$AH.p(i);
    else {
      const o = new _i(n, this), a = o.u(this.options);
      o.p(i), this.T(a), this._$AH = o;
    }
  }
  _$AC(t) {
    let i = de.get(t.strings);
    return i === void 0 && de.set(t.strings, i = new ot(t)), i;
  }
  k(t) {
    ke(this._$AH) || (this._$AH = [], this._$AR());
    const i = this._$AH;
    let s, n = 0;
    for (const r of t)
      n === i.length ? i.push(s = new dt(this.S(nt()), this.S(nt()), this, this.options)) : s = i[n], s._$AI(r), n++;
    n < i.length && (this._$AR(s && s._$AB.nextSibling, n), i.length = n);
  }
  _$AR(t = this._$AA.nextSibling, i) {
    var s;
    for ((s = this._$AP) == null ? void 0 : s.call(this, !1, !0, i); t && t !== this._$AB; ) {
      const n = t.nextSibling;
      t.remove(), t = n;
    }
  }
  setConnected(t) {
    var i;
    this._$AM === void 0 && (this._$Cv = t, (i = this._$AP) == null || i.call(this, t));
  }
}
class Pt {
  get tagName() {
    return this.element.tagName;
  }
  get _$AU() {
    return this._$AM._$AU;
  }
  constructor(t, i, s, n, r) {
    this.type = 1, this._$AH = f, this._$AN = void 0, this.element = t, this.name = i, this._$AM = n, this.options = r, s.length > 2 || s[0] !== "" || s[1] !== "" ? (this._$AH = Array(s.length - 1).fill(new String()), this.strings = s) : this._$AH = f;
  }
  _$AI(t, i = this, s, n) {
    const r = this.strings;
    let o = !1;
    if (r === void 0)
      t = J(this, t, i, 0), o = !rt(t) || t !== this._$AH && t !== D, o && (this._$AH = t);
    else {
      const a = t;
      let l, c;
      for (t = r[0], l = 0; l < r.length - 1; l++)
        c = J(this, a[s + l], i, l), c === D && (c = this._$AH[l]), o || (o = !rt(c) || c !== this._$AH[l]), c === f ? t = f : t !== f && (t += (c ?? "") + r[l + 1]), this._$AH[l] = c;
    }
    o && !n && this.j(t);
  }
  j(t) {
    t === f ? this.element.removeAttribute(this.name) : this.element.setAttribute(this.name, t ?? "");
  }
}
class Ai extends Pt {
  constructor() {
    super(...arguments), this.type = 3;
  }
  j(t) {
    this.element[this.name] = t === f ? void 0 : t;
  }
}
class Si extends Pt {
  constructor() {
    super(...arguments), this.type = 4;
  }
  j(t) {
    this.element.toggleAttribute(this.name, !!t && t !== f);
  }
}
class Ei extends Pt {
  constructor(t, i, s, n, r) {
    super(t, i, s, n, r), this.type = 5;
  }
  _$AI(t, i = this) {
    if ((t = J(this, t, i, 0) ?? f) === D)
      return;
    const s = this._$AH, n = t === f && s !== f || t.capture !== s.capture || t.once !== s.once || t.passive !== s.passive, r = t !== f && (s === f || n);
    n && this.element.removeEventListener(this.name, this, s), r && this.element.addEventListener(this.name, this, t), this._$AH = t;
  }
  handleEvent(t) {
    var i;
    typeof this._$AH == "function" ? this._$AH.call(((i = this.options) == null ? void 0 : i.host) ?? this.element, t) : this._$AH.handleEvent(t);
  }
}
class ki {
  constructor(t, i, s) {
    this.element = t, this.type = 6, this._$AN = void 0, this._$AM = i, this.options = s;
  }
  get _$AU() {
    return this._$AM._$AU;
  }
  _$AI(t) {
    J(this, t);
  }
}
const Dt = st.litHtmlPolyfillSupport;
Dt == null || Dt(ot, dt), (st.litHtmlVersions ?? (st.litHtmlVersions = [])).push("3.1.3");
const Ce = (e, t, i) => {
  const s = (i == null ? void 0 : i.renderBefore) ?? t;
  let n = s._$litPart$;
  if (n === void 0) {
    const r = (i == null ? void 0 : i.renderBefore) ?? null;
    s._$litPart$ = n = new dt(t.insertBefore(nt(), r), r, void 0, i ?? {});
  }
  return n._$AI(e), n;
};
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
class _ extends W {
  constructor() {
    super(...arguments), this.renderOptions = { host: this }, this._$Do = void 0;
  }
  createRenderRoot() {
    var i;
    const t = super.createRenderRoot();
    return (i = this.renderOptions).renderBefore ?? (i.renderBefore = t.firstChild), t;
  }
  update(t) {
    const i = this.render();
    this.hasUpdated || (this.renderOptions.isConnected = this.isConnected), super.update(t), this._$Do = Ce(i, this.renderRoot, this.renderOptions);
  }
  connectedCallback() {
    var t;
    super.connectedCallback(), (t = this._$Do) == null || t.setConnected(!0);
  }
  disconnectedCallback() {
    var t;
    super.disconnectedCallback(), (t = this._$Do) == null || t.setConnected(!1);
  }
  render() {
    return D;
  }
}
var _e;
_._$litElement$ = !0, _.finalized = !0, (_e = globalThis.litElementHydrateSupport) == null || _e.call(globalThis, { LitElement: _ });
const Ut = globalThis.litElementPolyfillSupport;
Ut == null || Ut({ LitElement: _ });
(globalThis.litElementVersions ?? (globalThis.litElementVersions = [])).push("4.0.5");
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const P = (e) => (t, i) => {
  i !== void 0 ? i.addInitializer(() => {
    customElements.define(e, t);
  }) : customElements.define(e, t);
};
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const Pi = { attribute: !0, type: String, converter: gt, reflect: !1, hasChanged: Ft }, Oi = (e = Pi, t, i) => {
  const { kind: s, metadata: n } = i;
  let r = globalThis.litPropertyMetadata.get(n);
  if (r === void 0 && globalThis.litPropertyMetadata.set(n, r = /* @__PURE__ */ new Map()), r.set(i.name, e), s === "accessor") {
    const { name: o } = i;
    return { set(a) {
      const l = t.get.call(this);
      t.set.call(this, a), this.requestUpdate(o, l, e);
    }, init(a) {
      return a !== void 0 && this.P(o, void 0, e), a;
    } };
  }
  if (s === "setter") {
    const { name: o } = i;
    return function(a) {
      const l = this[o];
      t.call(this, a), this.requestUpdate(o, l, e);
    };
  }
  throw Error("Unsupported decorator location: " + s);
};
function $(e) {
  return (t, i) => typeof i == "object" ? Oi(e, t, i) : ((s, n, r) => {
    const o = n.hasOwnProperty(r);
    return n.constructor.createProperty(r, o ? { ...s, wrapped: !0 } : s), o ? Object.getOwnPropertyDescriptor(n, r) : void 0;
  })(e, t, i);
}
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
function Ot(e) {
  return $({ ...e, state: !0, attribute: !1 });
}
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const ue = (e, t, i) => (i.configurable = !0, i.enumerable = !0, Reflect.decorate && typeof t != "object" && Object.defineProperty(e, t, i), i);
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
function Ci(e, t) {
  return (i, s, n) => {
    const r = (o) => {
      var a;
      return ((a = o.renderRoot) == null ? void 0 : a.querySelector(e)) ?? null;
    };
    if (t) {
      const { get: o, set: a } = typeof s == "object" ? i : n ?? (() => {
        const l = Symbol();
        return { get() {
          return this[l];
        }, set(c) {
          this[l] = c;
        } };
      })();
      return ue(i, s, { get() {
        let l = o.call(this);
        return l === void 0 && (l = r(this), (l !== null || this.hasUpdated) && a.call(this, l)), l;
      } });
    }
    return ue(i, s, { get() {
      return r(this);
    } });
  };
}
/**
 * @license
 * Copyright 2017 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const Te = { ATTRIBUTE: 1, CHILD: 2, PROPERTY: 3, BOOLEAN_ATTRIBUTE: 4, EVENT: 5, ELEMENT: 6 }, Me = (e) => (...t) => ({ _$litDirective$: e, values: t });
let Re = class {
  constructor(t) {
  }
  get _$AU() {
    return this._$AM._$AU;
  }
  _$AT(t, i, s) {
    this._$Ct = t, this._$AM = i, this._$Ci = s;
  }
  _$AS(t, i) {
    return this.update(t, i);
  }
  update(t, i) {
    return this.render(...i);
  }
};
/**
 * @license
 * Copyright 2018 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const lt = Me(class extends Re {
  constructor(e) {
    var t;
    if (super(e), e.type !== Te.ATTRIBUTE || e.name !== "class" || ((t = e.strings) == null ? void 0 : t.length) > 2)
      throw Error("`classMap()` can only be used in the `class` attribute and must be the only part in the attribute.");
  }
  render(e) {
    return " " + Object.keys(e).filter((t) => e[t]).join(" ") + " ";
  }
  update(e, [t]) {
    var s, n;
    if (this.st === void 0) {
      this.st = /* @__PURE__ */ new Set(), e.strings !== void 0 && (this.nt = new Set(e.strings.join(" ").split(/\s/).filter((r) => r !== "")));
      for (const r in t)
        t[r] && !((s = this.nt) != null && s.has(r)) && this.st.add(r);
      return this.render(t);
    }
    const i = e.element.classList;
    for (const r of this.st)
      r in t || (i.remove(r), this.st.delete(r));
    for (const r in t) {
      const o = !!t[r];
      o === this.st.has(r) || (n = this.nt) != null && n.has(r) || (o ? (i.add(r), this.st.add(r)) : (i.remove(r), this.st.delete(r)));
    }
    return D;
  }
});
function Le(e) {
  switch (e) {
    case 0:
      return "normal";
    case 1:
      return "magic";
    case 2:
      return "rare";
    case 3:
      return "unique";
    case 4:
      return "gem";
    case 5:
      return "currency";
    case 11:
      return "necropolis";
    default:
      return null;
  }
}
function je(e) {
  const [t = "", ...i] = e;
  return `${t.toUpperCase()}${i.join("")}`;
}
function ze(e, t) {
  if (e.displayMode !== 3)
    throw new Error(`Expected displayMode 3, got ${e.displayMode}`);
  const i = e.name.split(/\{(\d+)\}/g).map((s, n) => {
    var o;
    if (n % 2 === 0)
      return s;
    const r = (o = e.values[parseInt(s)]) == null ? void 0 : o[0];
    return r == null ? s : t ? t(r) : r;
  });
  return t ? i : i.join("");
}
function De() {
  document.querySelector('style[data-description="poe-custom-elements-font"]') || document.head.insertAdjacentHTML(
    "beforeend",
    `
            <style data-description="poe-custom-elements-font">
                @font-face {
				    font-family: 'fontin';
				    font-weight: normal;
				    font-style: normal;
				    src: url('/fontin.woff') format('woff');
			    }
            </style>
        `
  );
}
var Ti = Object.defineProperty, Mi = Object.getOwnPropertyDescriptor, Wt = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? Mi(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && Ti(t, i, n), n;
};
let at = class extends _ {
  willUpdate() {
    this.style.setProperty("--background-image", this.gemImageSrc());
  }
  render() {
    return u` <div class="highlight-ring"></div> `;
  }
  gemImageSrc() {
    const e = (t) => {
      var s, n;
      const i = (s = this.socketedItem) != null && s.support ? "Support" : "Skill";
      switch (t) {
        case "A":
          return this.socketedItem ? "socketAbyss" : "socketAbyssFull";
        case "B":
          return this.socketedItem ? `intFull${i}` : "int";
        case "G":
          return this.socketedItem ? `dexFull${i}` : "dex";
        case "R":
          return this.socketedItem ? `strFull${i}` : "str";
        case "W": {
          const r = (n = this.socketedItem) == null ? void 0 : n.colour, o = (a) => `gen${je(e(a))}`;
          if (!this.socketedItem)
            return "gen";
          if (!r)
            return o("B");
          switch (r) {
            case "R":
            case "G":
            case "B":
              return o(r);
            default:
              return o("B");
          }
        }
      }
    };
    return `url('/poe-images/${e(this.kind)}.png')`;
  }
};
at.styles = S`
		:host {
			--background-image: '(computed) Image of empty or full socket';
			display: inline-block;
			width: var(--cell-size);
			height: var(--cell-size);
			position: relative;
			background: var(--background-image);
			background-size: 100%;
		}

		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		.highlight-ring {
			display: none;
			top: 50%;
			left: 50%;
			transform: translate(-50%, -50%);
			position: absolute;
			content: '';
			width: calc(var(--cell-size) / var(--default-cell-size) * 31);
			height: calc(var(--cell-size) / var(--default-cell-size) * 31);
			border-radius: 50%;

			box-shadow: 0px 0px 4px 2px #fff;
		}

		:host(:hover) .highlight-ring {
			display: initial;
		}

		img {
			width: 100%;
			display: block;
			height: var(--cell-size);
		}
	`;
Wt([
  $({ reflect: !0 })
], at.prototype, "kind", 2);
Wt([
  $({ type: Object })
], at.prototype, "socketedItem", 2);
at = Wt([
  P("poe-item-socket")
], at);
var Ri = Object.defineProperty, Li = Object.getOwnPropertyDescriptor, Ct = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? Li(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && Ri(t, i, n), n;
};
let G = class extends _ {
  constructor() {
    super(...arguments), this.socketedItems = [], this.sockets = [];
  }
  render() {
    return u`<ul
			class=${lt({
      "item-width--w1": this.w === 1,
      "item-width--w2": this.w === 2
    })}
		>
			${Object.values(
      Object.groupBy(
        this.sockets.map((e, t) => ({
          socketNo: t + 1,
          ...e,
          socketedItem: this.socketedItems.find((i) => i.socket === t)
        })),
        (e) => e.group
      )
    ).flatMap((e = []) => e.map(
      (t, i) => u`<li style="grid-area: s${t.socketNo}">
						<poe-item-socket
							@pointerenter=${() => this.onSocketPointerEnter(t.socketedItem ?? null)}
							@pointerleave=${this.onSocketPointerLeave}
							.socketedItem=${t.socketedItem}
							.kind=${t.sColour}
						></poe-item-socket>
						${i === e.length - 1 ? f : u`<div
									class=${lt({
        "socket-link": !0,
        [`socket-link--${this.socketLinkDirection(t.socketNo)}`]: !0
      })}
							  >
									<img class="socket-link-img" src="/poe-images/Socket_Link_Horizontal.png" />
							  </div>`}
					</li>`
    ))}
		</ul>`;
  }
  onSocketPointerEnter(e) {
    this.dispatchEvent(new CustomEvent("hovered-socketed-item-changed", { detail: e }));
  }
  onSocketPointerLeave() {
    this.dispatchEvent(new CustomEvent("hovered-socketed-item-changed", { detail: null }));
  }
  socketLinkDirection(e) {
    switch (this.w) {
      case 1:
        return "top-to-bottom";
      case 2:
        switch (e) {
          case 1:
            return "left-to-right";
          case 2:
            return "top-to-bottom";
          case 3:
            return "right-to-left";
          case 4:
            return "top-to-bottom";
          case 5:
            return "left-to-right";
          default:
            throw new Error(`SocketNo can be 1 | 2 | 3 | 4 | 5, but not ${e}`);
        }
      default:
        throw new Error(`Item width can be 1 cell or 2 cells, but not ${this.w}`);
    }
  }
};
G.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			--default-socket-link-image-width-px: 38;
			--socket-link-img-width: calc(
				var(--default-socket-link-image-width-px) * var(--cell-size) / var(--default-cell-size)
			);
			display: inline-block;
			display: flex;
			align-items: center;
			justify-content: center;
		}

		.item-width--w1 {
			grid-template-areas:
				's1'
				's2'
				's3'
				's4';
		}

		.item-width--w2 {
			grid-template-areas:
				's1 s2'
				's4 s3'
				's5 s6';
		}

		ul {
			list-style: none;
			display: grid;
			grid-template-areas:
				's1 s2'
				's4 s3'
				's5 s6';
		}

		li {
			position: relative;
			display: flex;
			justify-content: center;
			align-items: center;
		}

		.socket-link {
			position: absolute;
			z-index: 5;
		}

		.socket-link-img {
			width: var(--socket-link-img-width);
		}

		.socket-link--left-to-right {
			right: 0px;
			transform: translateX(50%);
		}

		.socket-link--right-to-left {
			left: 0px;
			transform: translateX(-50%);
		}

		.socket-link--top-to-bottom {
			rotate: -90deg;
			bottom: 0;
			translate: 2px 50%;
		}
	`;
Ct([
  $({ type: Array })
], G.prototype, "socketedItems", 2);
Ct([
  $({ type: Array })
], G.prototype, "sockets", 2);
Ct([
  $({ type: Number })
], G.prototype, "w", 2);
G = Ct([
  P("poe-socket-chain")
], G);
const ji = ["top", "right", "bottom", "left"], pe = ["start", "end"], fe = /* @__PURE__ */ ji.reduce((e, t) => e.concat(t, t + "-" + pe[0], t + "-" + pe[1]), []), Ht = Math.min, Y = Math.max, vt = Math.round, U = (e) => ({
  x: e,
  y: e
}), zi = {
  left: "right",
  right: "left",
  bottom: "top",
  top: "bottom"
}, Di = {
  start: "end",
  end: "start"
};
function me(e, t, i) {
  return Y(e, Ht(t, i));
}
function Tt(e, t) {
  return typeof e == "function" ? e(t) : e;
}
function Q(e) {
  return e.split("-")[0];
}
function z(e) {
  return e.split("-")[1];
}
function Ue(e) {
  return e === "x" ? "y" : "x";
}
function Ne(e) {
  return e === "y" ? "height" : "width";
}
function Mt(e) {
  return ["top", "bottom"].includes(Q(e)) ? "y" : "x";
}
function He(e) {
  return Ue(Mt(e));
}
function Ui(e, t, i) {
  i === void 0 && (i = !1);
  const s = z(e), n = He(e), r = Ne(n);
  let o = n === "x" ? s === (i ? "end" : "start") ? "right" : "left" : s === "start" ? "bottom" : "top";
  return t.reference[r] > t.floating[r] && (o = ge(o)), [o, ge(o)];
}
function Ni(e) {
  return e.replace(/start|end/g, (t) => Di[t]);
}
function ge(e) {
  return e.replace(/left|right|bottom|top/g, (t) => zi[t]);
}
function Hi(e) {
  return {
    top: 0,
    right: 0,
    bottom: 0,
    left: 0,
    ...e
  };
}
function Ii(e) {
  return typeof e != "number" ? Hi(e) : {
    top: e,
    right: e,
    bottom: e,
    left: e
  };
}
function wt(e) {
  return {
    ...e,
    top: e.y,
    left: e.x,
    right: e.x + e.width,
    bottom: e.y + e.height
  };
}
function ye(e, t, i) {
  let {
    reference: s,
    floating: n
  } = e;
  const r = Mt(t), o = He(t), a = Ne(o), l = Q(t), c = r === "y", d = s.x + s.width / 2 - n.width / 2, h = s.y + s.height / 2 - n.height / 2, m = s[a] / 2 - n[a] / 2;
  let p;
  switch (l) {
    case "top":
      p = {
        x: d,
        y: s.y - n.height
      };
      break;
    case "bottom":
      p = {
        x: d,
        y: s.y + s.height
      };
      break;
    case "right":
      p = {
        x: s.x + s.width,
        y: h
      };
      break;
    case "left":
      p = {
        x: s.x - n.width,
        y: h
      };
      break;
    default:
      p = {
        x: s.x,
        y: s.y
      };
  }
  switch (z(t)) {
    case "start":
      p[o] -= m * (i && c ? -1 : 1);
      break;
    case "end":
      p[o] += m * (i && c ? -1 : 1);
      break;
  }
  return p;
}
const Bi = async (e, t, i) => {
  const {
    placement: s = "bottom",
    strategy: n = "absolute",
    middleware: r = [],
    platform: o
  } = i, a = r.filter(Boolean), l = await (o.isRTL == null ? void 0 : o.isRTL(t));
  let c = await o.getElementRects({
    reference: e,
    floating: t,
    strategy: n
  }), {
    x: d,
    y: h
  } = ye(c, s, l), m = s, p = {}, g = 0;
  for (let v = 0; v < a.length; v++) {
    const {
      name: y,
      fn: w
    } = a[v], {
      x: b,
      y: A,
      data: O,
      reset: x
    } = await w({
      x: d,
      y: h,
      initialPlacement: s,
      placement: m,
      strategy: n,
      middlewareData: p,
      rects: c,
      platform: o,
      elements: {
        reference: e,
        floating: t
      }
    });
    d = b ?? d, h = A ?? h, p = {
      ...p,
      [y]: {
        ...p[y],
        ...O
      }
    }, x && g <= 50 && (g++, typeof x == "object" && (x.placement && (m = x.placement), x.rects && (c = x.rects === !0 ? await o.getElementRects({
      reference: e,
      floating: t,
      strategy: n
    }) : x.rects), {
      x: d,
      y: h
    } = ye(c, m, l)), v = -1);
  }
  return {
    x: d,
    y: h,
    placement: m,
    strategy: n,
    middlewareData: p
  };
};
async function Ie(e, t) {
  var i;
  t === void 0 && (t = {});
  const {
    x: s,
    y: n,
    platform: r,
    rects: o,
    elements: a,
    strategy: l
  } = e, {
    boundary: c = "clippingAncestors",
    rootBoundary: d = "viewport",
    elementContext: h = "floating",
    altBoundary: m = !1,
    padding: p = 0
  } = Tt(t, e), g = Ii(p), y = a[m ? h === "floating" ? "reference" : "floating" : h], w = wt(await r.getClippingRect({
    element: (i = await (r.isElement == null ? void 0 : r.isElement(y))) == null || i ? y : y.contextElement || await (r.getDocumentElement == null ? void 0 : r.getDocumentElement(a.floating)),
    boundary: c,
    rootBoundary: d,
    strategy: l
  })), b = h === "floating" ? {
    ...o.floating,
    x: s,
    y: n
  } : o.reference, A = await (r.getOffsetParent == null ? void 0 : r.getOffsetParent(a.floating)), O = await (r.isElement == null ? void 0 : r.isElement(A)) ? await (r.getScale == null ? void 0 : r.getScale(A)) || {
    x: 1,
    y: 1
  } : {
    x: 1,
    y: 1
  }, x = wt(r.convertOffsetParentRelativeRectToViewportRelativeRect ? await r.convertOffsetParentRelativeRectToViewportRelativeRect({
    elements: a,
    rect: b,
    offsetParent: A,
    strategy: l
  }) : b);
  return {
    top: (w.top - x.top + g.top) / O.y,
    bottom: (x.bottom - w.bottom + g.bottom) / O.y,
    left: (w.left - x.left + g.left) / O.x,
    right: (x.right - w.right + g.right) / O.x
  };
}
function Vi(e, t, i) {
  return (e ? [...i.filter((n) => z(n) === e), ...i.filter((n) => z(n) !== e)] : i.filter((n) => Q(n) === n)).filter((n) => e ? z(n) === e || (t ? Ni(n) !== n : !1) : !0);
}
const qi = function(e) {
  return e === void 0 && (e = {}), {
    name: "autoPlacement",
    options: e,
    async fn(t) {
      var i, s, n;
      const {
        rects: r,
        middlewareData: o,
        placement: a,
        platform: l,
        elements: c
      } = t, {
        crossAxis: d = !1,
        alignment: h,
        allowedPlacements: m = fe,
        autoAlignment: p = !0,
        ...g
      } = Tt(e, t), v = h !== void 0 || m === fe ? Vi(h || null, p, m) : m, y = await Ie(t, g), w = ((i = o.autoPlacement) == null ? void 0 : i.index) || 0, b = v[w];
      if (b == null)
        return {};
      const A = Ui(b, r, await (l.isRTL == null ? void 0 : l.isRTL(c.floating)));
      if (a !== b)
        return {
          reset: {
            placement: v[0]
          }
        };
      const O = [y[Q(b)], y[A[0]], y[A[1]]], x = [...((s = o.autoPlacement) == null ? void 0 : s.overflows) || [], {
        placement: b,
        overflows: O
      }], Kt = v[w + 1];
      if (Kt)
        return {
          data: {
            index: w + 1,
            overflows: x
          },
          reset: {
            placement: Kt
          }
        };
      const Qt = x.map((C) => {
        const Z = z(C.placement);
        return [C.placement, Z && d ? (
          // Check along the mainAxis and main crossAxis side.
          C.overflows.slice(0, 2).reduce((li, ai) => li + ai, 0)
        ) : (
          // Check only the mainAxis.
          C.overflows[0]
        ), C.overflows];
      }).sort((C, Z) => C[1] - Z[1]), Zt = ((n = Qt.filter((C) => C[2].slice(
        0,
        // Aligned placements should not check their opposite crossAxis
        // side.
        z(C[0]) ? 2 : 3
      ).every((Z) => Z <= 0))[0]) == null ? void 0 : n[0]) || Qt[0][0];
      return Zt !== a ? {
        data: {
          index: w + 1,
          overflows: x
        },
        reset: {
          placement: Zt
        }
      } : {};
    }
  };
};
async function Fi(e, t) {
  const {
    placement: i,
    platform: s,
    elements: n
  } = e, r = await (s.isRTL == null ? void 0 : s.isRTL(n.floating)), o = Q(i), a = z(i), l = Mt(i) === "y", c = ["left", "top"].includes(o) ? -1 : 1, d = r && l ? -1 : 1, h = Tt(t, e);
  let {
    mainAxis: m,
    crossAxis: p,
    alignmentAxis: g
  } = typeof h == "number" ? {
    mainAxis: h,
    crossAxis: 0,
    alignmentAxis: null
  } : {
    mainAxis: 0,
    crossAxis: 0,
    alignmentAxis: null,
    ...h
  };
  return a && typeof g == "number" && (p = a === "end" ? g * -1 : g), l ? {
    x: p * d,
    y: m * c
  } : {
    x: m * c,
    y: p * d
  };
}
const Wi = function(e) {
  return e === void 0 && (e = 0), {
    name: "offset",
    options: e,
    async fn(t) {
      var i, s;
      const {
        x: n,
        y: r,
        placement: o,
        middlewareData: a
      } = t, l = await Fi(t, e);
      return o === ((i = a.offset) == null ? void 0 : i.placement) && (s = a.arrow) != null && s.alignmentOffset ? {} : {
        x: n + l.x,
        y: r + l.y,
        data: {
          ...l,
          placement: o
        }
      };
    }
  };
}, Yi = function(e) {
  return e === void 0 && (e = {}), {
    name: "shift",
    options: e,
    async fn(t) {
      const {
        x: i,
        y: s,
        placement: n
      } = t, {
        mainAxis: r = !0,
        crossAxis: o = !1,
        limiter: a = {
          fn: (y) => {
            let {
              x: w,
              y: b
            } = y;
            return {
              x: w,
              y: b
            };
          }
        },
        ...l
      } = Tt(e, t), c = {
        x: i,
        y: s
      }, d = await Ie(t, l), h = Mt(Q(n)), m = Ue(h);
      let p = c[m], g = c[h];
      if (r) {
        const y = m === "y" ? "top" : "left", w = m === "y" ? "bottom" : "right", b = p + d[y], A = p - d[w];
        p = me(b, p, A);
      }
      if (o) {
        const y = h === "y" ? "top" : "left", w = h === "y" ? "bottom" : "right", b = g + d[y], A = g - d[w];
        g = me(b, g, A);
      }
      const v = a.fn({
        ...t,
        [m]: p,
        [h]: g
      });
      return {
        ...v,
        data: {
          x: v.x - i,
          y: v.y - s
        }
      };
    }
  };
};
function N(e) {
  return Be(e) ? (e.nodeName || "").toLowerCase() : "#document";
}
function E(e) {
  var t;
  return (e == null || (t = e.ownerDocument) == null ? void 0 : t.defaultView) || window;
}
function H(e) {
  var t;
  return (t = (Be(e) ? e.ownerDocument : e.document) || window.document) == null ? void 0 : t.documentElement;
}
function Be(e) {
  return e instanceof Node || e instanceof E(e).Node;
}
function R(e) {
  return e instanceof Element || e instanceof E(e).Element;
}
function T(e) {
  return e instanceof HTMLElement || e instanceof E(e).HTMLElement;
}
function ve(e) {
  return typeof ShadowRoot > "u" ? !1 : e instanceof ShadowRoot || e instanceof E(e).ShadowRoot;
}
function ut(e) {
  const {
    overflow: t,
    overflowX: i,
    overflowY: s,
    display: n
  } = k(e);
  return /auto|scroll|overlay|hidden|clip/.test(t + s + i) && !["inline", "contents"].includes(n);
}
function Xi(e) {
  return ["table", "td", "th"].includes(N(e));
}
function Yt(e) {
  const t = Xt(), i = k(e);
  return i.transform !== "none" || i.perspective !== "none" || (i.containerType ? i.containerType !== "normal" : !1) || !t && (i.backdropFilter ? i.backdropFilter !== "none" : !1) || !t && (i.filter ? i.filter !== "none" : !1) || ["transform", "perspective", "filter"].some((s) => (i.willChange || "").includes(s)) || ["paint", "layout", "strict", "content"].some((s) => (i.contain || "").includes(s));
}
function Ji(e) {
  let t = K(e);
  for (; T(t) && !Rt(t); ) {
    if (Yt(t))
      return t;
    t = K(t);
  }
  return null;
}
function Xt() {
  return typeof CSS > "u" || !CSS.supports ? !1 : CSS.supports("-webkit-backdrop-filter", "none");
}
function Rt(e) {
  return ["html", "body", "#document"].includes(N(e));
}
function k(e) {
  return E(e).getComputedStyle(e);
}
function Lt(e) {
  return R(e) ? {
    scrollLeft: e.scrollLeft,
    scrollTop: e.scrollTop
  } : {
    scrollLeft: e.pageXOffset,
    scrollTop: e.pageYOffset
  };
}
function K(e) {
  if (N(e) === "html")
    return e;
  const t = (
    // Step into the shadow DOM of the parent of a slotted node.
    e.assignedSlot || // DOM Element detected.
    e.parentNode || // ShadowRoot detected.
    ve(e) && e.host || // Fallback.
    H(e)
  );
  return ve(t) ? t.host : t;
}
function Ve(e) {
  const t = K(e);
  return Rt(t) ? e.ownerDocument ? e.ownerDocument.body : e.body : T(t) && ut(t) ? t : Ve(t);
}
function It(e, t, i) {
  var s;
  t === void 0 && (t = []), i === void 0 && (i = !0);
  const n = Ve(e), r = n === ((s = e.ownerDocument) == null ? void 0 : s.body), o = E(n);
  return r ? t.concat(o, o.visualViewport || [], ut(n) ? n : [], o.frameElement && i ? It(o.frameElement) : []) : t.concat(n, It(n, [], i));
}
function qe(e) {
  const t = k(e);
  let i = parseFloat(t.width) || 0, s = parseFloat(t.height) || 0;
  const n = T(e), r = n ? e.offsetWidth : i, o = n ? e.offsetHeight : s, a = vt(i) !== r || vt(s) !== o;
  return a && (i = r, s = o), {
    width: i,
    height: s,
    $: a
  };
}
function Fe(e) {
  return R(e) ? e : e.contextElement;
}
function X(e) {
  const t = Fe(e);
  if (!T(t))
    return U(1);
  const i = t.getBoundingClientRect(), {
    width: s,
    height: n,
    $: r
  } = qe(t);
  let o = (r ? vt(i.width) : i.width) / s, a = (r ? vt(i.height) : i.height) / n;
  return (!o || !Number.isFinite(o)) && (o = 1), (!a || !Number.isFinite(a)) && (a = 1), {
    x: o,
    y: a
  };
}
const Gi = /* @__PURE__ */ U(0);
function We(e) {
  const t = E(e);
  return !Xt() || !t.visualViewport ? Gi : {
    x: t.visualViewport.offsetLeft,
    y: t.visualViewport.offsetTop
  };
}
function Ki(e, t, i) {
  return t === void 0 && (t = !1), !i || t && i !== E(e) ? !1 : t;
}
function ct(e, t, i, s) {
  t === void 0 && (t = !1), i === void 0 && (i = !1);
  const n = e.getBoundingClientRect(), r = Fe(e);
  let o = U(1);
  t && (s ? R(s) && (o = X(s)) : o = X(e));
  const a = Ki(r, i, s) ? We(r) : U(0);
  let l = (n.left + a.x) / o.x, c = (n.top + a.y) / o.y, d = n.width / o.x, h = n.height / o.y;
  if (r) {
    const m = E(r), p = s && R(s) ? E(s) : s;
    let g = m, v = g.frameElement;
    for (; v && s && p !== g; ) {
      const y = X(v), w = v.getBoundingClientRect(), b = k(v), A = w.left + (v.clientLeft + parseFloat(b.paddingLeft)) * y.x, O = w.top + (v.clientTop + parseFloat(b.paddingTop)) * y.y;
      l *= y.x, c *= y.y, d *= y.x, h *= y.y, l += A, c += O, g = E(v), v = g.frameElement;
    }
  }
  return wt({
    width: d,
    height: h,
    x: l,
    y: c
  });
}
const Qi = [":popover-open", ":modal"];
function Ye(e) {
  return Qi.some((t) => {
    try {
      return e.matches(t);
    } catch {
      return !1;
    }
  });
}
function Zi(e) {
  let {
    elements: t,
    rect: i,
    offsetParent: s,
    strategy: n
  } = e;
  const r = n === "fixed", o = H(s), a = t ? Ye(t.floating) : !1;
  if (s === o || a && r)
    return i;
  let l = {
    scrollLeft: 0,
    scrollTop: 0
  }, c = U(1);
  const d = U(0), h = T(s);
  if ((h || !h && !r) && ((N(s) !== "body" || ut(o)) && (l = Lt(s)), T(s))) {
    const m = ct(s);
    c = X(s), d.x = m.x + s.clientLeft, d.y = m.y + s.clientTop;
  }
  return {
    width: i.width * c.x,
    height: i.height * c.y,
    x: i.x * c.x - l.scrollLeft * c.x + d.x,
    y: i.y * c.y - l.scrollTop * c.y + d.y
  };
}
function ts(e) {
  return Array.from(e.getClientRects());
}
function Xe(e) {
  return ct(H(e)).left + Lt(e).scrollLeft;
}
function es(e) {
  const t = H(e), i = Lt(e), s = e.ownerDocument.body, n = Y(t.scrollWidth, t.clientWidth, s.scrollWidth, s.clientWidth), r = Y(t.scrollHeight, t.clientHeight, s.scrollHeight, s.clientHeight);
  let o = -i.scrollLeft + Xe(e);
  const a = -i.scrollTop;
  return k(s).direction === "rtl" && (o += Y(t.clientWidth, s.clientWidth) - n), {
    width: n,
    height: r,
    x: o,
    y: a
  };
}
function is(e, t) {
  const i = E(e), s = H(e), n = i.visualViewport;
  let r = s.clientWidth, o = s.clientHeight, a = 0, l = 0;
  if (n) {
    r = n.width, o = n.height;
    const c = Xt();
    (!c || c && t === "fixed") && (a = n.offsetLeft, l = n.offsetTop);
  }
  return {
    width: r,
    height: o,
    x: a,
    y: l
  };
}
function ss(e, t) {
  const i = ct(e, !0, t === "fixed"), s = i.top + e.clientTop, n = i.left + e.clientLeft, r = T(e) ? X(e) : U(1), o = e.clientWidth * r.x, a = e.clientHeight * r.y, l = n * r.x, c = s * r.y;
  return {
    width: o,
    height: a,
    x: l,
    y: c
  };
}
function we(e, t, i) {
  let s;
  if (t === "viewport")
    s = is(e, i);
  else if (t === "document")
    s = es(H(e));
  else if (R(t))
    s = ss(t, i);
  else {
    const n = We(e);
    s = {
      ...t,
      x: t.x - n.x,
      y: t.y - n.y
    };
  }
  return wt(s);
}
function Je(e, t) {
  const i = K(e);
  return i === t || !R(i) || Rt(i) ? !1 : k(i).position === "fixed" || Je(i, t);
}
function ns(e, t) {
  const i = t.get(e);
  if (i)
    return i;
  let s = It(e, [], !1).filter((a) => R(a) && N(a) !== "body"), n = null;
  const r = k(e).position === "fixed";
  let o = r ? K(e) : e;
  for (; R(o) && !Rt(o); ) {
    const a = k(o), l = Yt(o);
    !l && a.position === "fixed" && (n = null), (r ? !l && !n : !l && a.position === "static" && !!n && ["absolute", "fixed"].includes(n.position) || ut(o) && !l && Je(e, o)) ? s = s.filter((d) => d !== o) : n = a, o = K(o);
  }
  return t.set(e, s), s;
}
function rs(e) {
  let {
    element: t,
    boundary: i,
    rootBoundary: s,
    strategy: n
  } = e;
  const o = [...i === "clippingAncestors" ? ns(t, this._c) : [].concat(i), s], a = o[0], l = o.reduce((c, d) => {
    const h = we(t, d, n);
    return c.top = Y(h.top, c.top), c.right = Ht(h.right, c.right), c.bottom = Ht(h.bottom, c.bottom), c.left = Y(h.left, c.left), c;
  }, we(t, a, n));
  return {
    width: l.right - l.left,
    height: l.bottom - l.top,
    x: l.left,
    y: l.top
  };
}
function os(e) {
  const {
    width: t,
    height: i
  } = qe(e);
  return {
    width: t,
    height: i
  };
}
function ls(e, t, i) {
  const s = T(t), n = H(t), r = i === "fixed", o = ct(e, !0, r, t);
  let a = {
    scrollLeft: 0,
    scrollTop: 0
  };
  const l = U(0);
  if (s || !s && !r)
    if ((N(t) !== "body" || ut(n)) && (a = Lt(t)), s) {
      const h = ct(t, !0, r, t);
      l.x = h.x + t.clientLeft, l.y = h.y + t.clientTop;
    } else
      n && (l.x = Xe(n));
  const c = o.left + a.scrollLeft - l.x, d = o.top + a.scrollTop - l.y;
  return {
    x: c,
    y: d,
    width: o.width,
    height: o.height
  };
}
function be(e, t) {
  return !T(e) || k(e).position === "fixed" ? null : t ? t(e) : e.offsetParent;
}
function Ge(e, t) {
  const i = E(e);
  if (!T(e) || Ye(e))
    return i;
  let s = be(e, t);
  for (; s && Xi(s) && k(s).position === "static"; )
    s = be(s, t);
  return s && (N(s) === "html" || N(s) === "body" && k(s).position === "static" && !Yt(s)) ? i : s || Ji(e) || i;
}
const as = async function(e) {
  const t = this.getOffsetParent || Ge, i = this.getDimensions;
  return {
    reference: ls(e.reference, await t(e.floating), e.strategy),
    floating: {
      x: 0,
      y: 0,
      ...await i(e.floating)
    }
  };
};
function cs(e) {
  return k(e).direction === "rtl";
}
const hs = {
  convertOffsetParentRelativeRectToViewportRelativeRect: Zi,
  getDocumentElement: H,
  getClippingRect: rs,
  getOffsetParent: Ge,
  getElementRects: as,
  getClientRects: ts,
  getDimensions: os,
  getScale: X,
  isElement: R,
  isRTL: cs
}, ds = qi, us = Yi, ps = (e, t, i) => {
  const s = /* @__PURE__ */ new Map(), n = {
    platform: hs,
    ...i
  }, r = {
    ...n.platform,
    _c: s
  };
  return Bi(e, t, {
    ...n,
    platform: r
  });
};
var fs = Object.defineProperty, ms = Object.getOwnPropertyDescriptor, Jt = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? ms(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && fs(t, i, n), n;
};
const pt = ["pointerenter", "focus", "click"], $e = ["pointerleave", "blur"];
let F = class extends _ {
  constructor() {
    super(), this.showing = !1, this.offset = 4, this._target = null, this.show = () => {
      this.style.cssText = "", ps(this.target, this, {
        strategy: "fixed",
        middleware: [
          Wi(this.offset),
          us(),
          ds({ allowedPlacements: ["bottom", "top", "top-start", "left-start"] })
        ]
      }).then(({ x: e, y: t }) => {
        this.style.left = `${e}px`, this.style.top = `${t}px`;
      }), this.showing = !0;
    }, this.hide = () => {
      this.showing = !1;
    }, this.finishHide = () => {
      this.showing || (this.style.display = "none");
    }, this.addEventListener("transitionend", this.finishHide);
  }
  // Lazy creation
  static lazy(e, t) {
    const i = () => {
      const s = document.createElement("simple-tooltip");
      t(s), e.parentNode.insertBefore(s, e.nextSibling), s.show(), pt.forEach((n) => e.removeEventListener(n, i));
    };
    pt.forEach((s) => e.addEventListener(s, i));
  }
  connectedCallback() {
    super.connectedCallback(), this.target ?? (this.target = this.previousElementSibling), this.finishHide();
  }
  get target() {
    return this._target;
  }
  set target(e) {
    this.target && (pt.forEach((t) => this.target.removeEventListener(t, this.show)), $e.forEach((t) => this.target.removeEventListener(t, this.hide))), e && (pt.forEach((t) => e.addEventListener(t, this.show)), $e.forEach((t) => e.addEventListener(t, this.hide))), this._target = e;
  }
  render() {
    return u`<slot></slot>`;
  }
};
F.styles = S`
		:host {
			/* Position fixed to help ensure the tooltip is "on top" */
			position: fixed;
			/*border: 1px solid darkgray;
			background-color: rgba(0, 0, 0, 0.8);
            
            */
			padding: 4px;
			border-radius: 4px;
			display: inline-block;
			pointer-events: none;
			z-index: 900;

			/* Animate in */
			opacity: 0;
			transform: scale(0.75);
			transition: opacity, transform;
			transition-duration: 0.33s;
		}

		:host([showing]) {
			opacity: 1;
			transform: scale(1);
		}
	`;
Jt([
  $({ reflect: !0, type: Boolean })
], F.prototype, "showing", 2);
Jt([
  $({ type: Number })
], F.prototype, "offset", 2);
F = Jt([
  P("simple-tooltip")
], F);
var gs = Object.defineProperty, ys = Object.getOwnPropertyDescriptor, Ke = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? ys(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && gs(t, i, n), n;
};
let bt = class extends _ {
  constructor() {
    super(...arguments), this.showing = !1;
  }
  showWithAutohide() {
    this.showing = !0, setTimeout(() => {
      this.showing = !1;
    }, 2e3);
  }
  render() {
    return u`
			<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 32 32">
				<path
					fill="#d6d024"
					d="M4 20v2h4.586L2 28.586L3.414 30L10 23.414V28h2v-8zm25-8l-2-6h-2v10h2v-6l2 6h2V6h-2zm-7.666-6h-2.667A1.67 1.67 0 0 0 17 7.667v6.667A1.67 1.67 0 0 0 18.666 16h2.667A1.67 1.67 0 0 0 23 14.334V7.667A1.67 1.67 0 0 0 21.334 6M21 14h-2V8h2zM9 7.667V10a2 2 0 0 0 2 2h2v2H9v2h4.334A1.67 1.67 0 0 0 15 14.334V12a2 2 0 0 0-2-2h-2V8h4V6h-4.334A1.67 1.67 0 0 0 9 7.667M5 14H3v-2H1v2.334A1.67 1.67 0 0 0 2.667 16h2.667A1.67 1.67 0 0 0 7 14.334V6H5z"
				/>
			</svg>
		`;
  }
};
bt.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			position: absolute;
			top: 100px;
			right: 0px;
			color: yellow;
			opacity: 0;
			transform: scale(0.75);
			z-index: 20;
			pointer-events: none;
			transition: opacity, transform, top;
			transition-duration: 0.2s;
		}

		:host([showing]) {
			opacity: 1;
			transform: scale(1);
			top: 0px;
		}
	`;
Ke([
  $({ type: Boolean, reflect: !0 })
], bt.prototype, "showing", 2);
bt = Ke([
  P("tooltip-json-icon")
], bt);
var vs = Object.defineProperty, ws = Object.getOwnPropertyDescriptor, Qe = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? ws(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && vs(t, i, n), n;
};
let $t = class extends _ {
  constructor() {
    super(...arguments), this.kind = "rare";
  }
  willUpdate(e) {
    e.has("kind") && this.style.setProperty("--separator-background-image", `url(/poe-images/separator-${this.kind}.png)`);
  }
};
$t.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			--separator-background-image: url(/poe-images/separator-rare.png);
			display: block;
			height: 7.91075px;
			padding-block: 1px;
			background: var(--separator-background-image) center no-repeat;
		}
	`;
Qe([
  $({ reflect: !0 })
], $t.prototype, "kind", 2);
$t = Qe([
  P("poe-separator")
], $t);
var bs = Object.defineProperty, $s = Object.getOwnPropertyDescriptor, Ze = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? $s(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && bs(t, i, n), n;
};
let xt = class extends _ {
  render() {
    return this.property ? u`<div class="property">${_s(this.property)}</div>` : u`<p style="color: red">No item property provided</p>`;
  }
};
xt.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		.property {
			color: #7f7f7f;
		}
		.value {
			color: #fff;
		}
		.fire {
			color: #960000;
		}
		.cold {
			color: #366492;
		}
		.lightning {
			color: gold;
		}
		.augmented {
			color: #88f;
		}
	`;
Ze([
  $({ type: Object })
], xt.prototype, "property", 2);
xt = Ze([
  P("poe-item-property")
], xt);
function xs(e) {
  switch (e) {
    case 1:
      return "augmented";
    case 4:
      return "fire";
    case 5:
      return "cold";
    case 6:
      return "lightning";
    default:
      return "";
  }
}
function _s(e) {
  if (e.displayMode === 0) {
    if (!e.values.length)
      return e.name;
    const t = e.values.map((i, s) => u`<span
					class=${lt({
      value: !0,
      [`${xs(i[1])}`]: !0
    })}
					>${i[0]}</span
				>${s === e.values.length - 1 ? f : ","} `);
    return u`${e.name}: ${t}`;
  }
  return e.displayMode === 3 ? ze(e, (t) => u`<span class="value">${t}</span>`) : f;
}
var As = Object.defineProperty, Ss = Object.getOwnPropertyDescriptor, ti = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? Ss(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && As(t, i, n), n;
};
let _t = class extends _ {
  constructor() {
    super(...arguments), this.requirements = [];
  }
  render() {
    return u`Requires
		${this.requirements.map((e, t) => {
      switch (e.displayMode) {
        case 0:
          return e.values.length ? u`${e.name} <span class="value">${e.values[0][0]}</span>${t === this.requirements.length - 1 ? f : ", "}` : e.name;
        case 1:
          return u`<span class="value">${e.values[0][0]}</span> ${e.name}
						${t === this.requirements.length - 1 ? f : ", "}`;
        default:
          return console.warn(
            `Unexpected displayMode for Requirement. Exptected 0 | 1, got: ${e.displayMode}`
          ), f;
      }
    })}`;
  }
};
_t.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			color: #7f7f7f;
		}

		.value {
			color: #fff;
		}
	`;
ti([
  $({ type: Array })
], _t.prototype, "requirements", 2);
_t = ti([
  P("poe-requirements")
], _t);
var Es = Object.defineProperty, ks = Object.getOwnPropertyDescriptor, ei = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? ks(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && Es(t, i, n), n;
};
let At = class extends _ {
  render() {
    return u`<div class="content">
			${[
      this.properties.length ? u`<ul>
							${this.properties.map((e) => u`<li><poe-item-property .property=${e}></poe-item-property></li>`)}
					  </ul> ` : f,
      this.item.itemLevel ? u`<p>Monster Level: <span class="monster-level">${this.item.itemLevel}</span></p>` : f,
      this.requirements.length ? u` <poe-requirements .requirements=${this.requirements}></poe-requirements>` : f,
      this.enchantments.length ? u`${this.enchantments.map(
        (e) => e.split(`
`).map((t) => u`<p class="enchant">${t}</p>`)
      )}` : f,
      this.implicits.length ? u` ${this.implicits.map((e) => u`<p class="implicitMod">${e}</p>`)} ` : f,
      this.explicits.length || this.crafts.length || this.fracturedMods.length ? u`
							${this.fracturedMods.map((e) => u`<p class="fractured">${e}</p>`)}
							${this.explicits.map((e) => u`<p class="explicitMod">${e}</p>`)}
							${this.crafts.map((e) => u`<p class="craft">${e}</p>`)}
					  ` : f,
      this.item.identified ? f : u` <p class="unidentified">Unidentified</p>`,
      this.item.corrupted ? u` <p class="corrupted">corrupted</p>` : f,
      this.item.flavourText ? u`${this.item.flavourText.map((e, t, i) => {
        var s;
        return t === i.length - 1 && e.includes("<default>") ? u`<p class="flavour-text_default mt-16">
									${((s = e.match(/{(.*?)}/)) == null ? void 0 : s[1]) ?? ""}
								</p>` : u`<p class="flavour-text">${e}</p>`;
      })}` : f,
      this.item.descrText ? u`<p class="description-text">${this.item.descrText}</p>` : f
    ].filter((e) => e !== f).flatMap(
      (e, t, i) => t === i.length - 1 ? [e] : [e, u`<poe-separator .kind=${Le(this.item.frameType) ?? "rare"}></poe-separator>`]
    )}
		</div>`;
  }
  firstUpdated() {
    this.style.setProperty("--description-width", window.getComputedStyle(this).width);
  }
  get enchantments() {
    return this.item.enchantMods ?? [];
  }
  get properties() {
    return this.item.properties ?? [];
  }
  get requirements() {
    return this.item.requirements ?? [];
  }
  get implicits() {
    return this.item.implicitMods ?? [];
  }
  get explicits() {
    return this.item.explicitMods ?? [];
  }
  get crafts() {
    return this.item.craftedMods ?? [];
  }
  get fracturedMods() {
    return this.item.fracturedMods ?? [];
  }
};
At.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		ul {
			list-style: none;
		}

		:host {
			font-family: fontin;
			display: inline-block;
			background-color: rgba(0, 0, 0, 0.8);
			width: 100%;
			height: 100%;
			color: #7f7f7f;
			--description-width: 300px;
			max-width: 500px;
		}

		.content {
			padding-top: 0.4rem;
			padding-bottom: 0.5rem;
			text-align: center;
			display: grid;
			gap: 0.05rem;
		}

		.augmented,
		.implicitMod,
		.explicitMod {
			color: #88f;
		}

		.craft,
		.enchant {
			color: #b4b4ff;
		}

		.fractured {
			color: #a29162;
		}

		.unidentified,
		.corrupted {
			color: #d20000;
		}

		.monster-level {
			color: #fff;
		}

		.flavour-text {
			color: #af6025;
			font-style: italic;
		}
		.flavour-text_default {
			color: #7f7f7f;
			font-style: italic;
		}

		.mt-16 {
			margin-top: 1rem;
		}

		.description-text {
			margin-inline: auto;
			display: flex;
			font-style: italic;
			max-width: var(--description-width);
			text-wrap: balance;
		}
	`;
ei([
  $({ type: Object })
], At.prototype, "item", 2);
At = ei([
  P("poe-item-info-content")
], At);
const Ps = S`
	--flavour-text-color: rgba(167, 90, 27, 1);
	--item-normal: 0, 0%, 78%;
	--item-rare: 60, 100%, 73%;
	--item-magic: 240, 100%, 77%;
	--item-unique-contrast: 25, 63%, 48%;
	--item-unique: 26, 65%, 42%;
	--item-gem: 177, 72%, 37%;
	--item-relic: 0, 0%, 78%;
	--item-currency: 42, 19%, 59%;
	--item-prophecy: 275, 100%, 65%;
	--item-divination: 0, 0%, 50%;
	--item-keystone: 46, 52%, 74%;
	--item-explicit: 240, 100%, 77%;
	--item-implicit: var(--item-explicit);
	--item-crafted: 240, 100%, 85%;
	--item-enchanted: var(--item-crafted);
	--item-fractured: 44, 26%, 51%;
	--item-corrupted: 0, 100%, 41%;
	--item-scourge: 20, 100%, 57%;
	--item-physical: 0, 0%, 58%;
	--item-fire: 0, 100%, 29%;
	--item-cold: 210, 46%, 39%;
	--item-lightning: 51, 100%, 50%;
	--item-chaos: 322, 73%, 47%;
	--item-augmented: rgb(138, 138, 255);
	--coolgrey-1000: 206, 24%, 7%;
	--item-necropolis: 44.35, 39%, 76.86%;
`, Os = S`
	.default {
		color: #7f7f7f;
	}
	.fractured {
		color: hsla(var(--item-fractured));
	}
	.enchanted {
		color: hsla(var(--item-enchanted));
	}
	.normal,
	.normalItem {
		color: hsla(var(--item-normal));
	}
	.magic,
	.magicItem {
		color: hsla(var(--item-magic));
	}
	.rare,
	.rareItem {
		color: hsla(var(--item-rare));
	}
	.unique,
	.uniqueItem {
		color: hsla(var(--item-unique));
	}
	.gem,
	.gemItem {
		color: hsla(var(--item-gem));
	}
	.currency,
	.currencyItem {
		color: hsla(var(--item-currency));
	}
	.necropolis,
	.necropolisItem {
		color: hsla(var(--item-necropolis));
	}
	.corrupted {
		color: hsla(var(--item-corrupted));
	}
	.divination {
		color: #0ebaff;
	}
	.augmented {
		color: var(--item-augmented);
	}
`;
var Cs = Object.defineProperty, Ts = Object.getOwnPropertyDescriptor, ii = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? Ts(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && Cs(t, i, n), n;
};
let St = class extends _ {
  willUpdate(e) {
    if (e.has("item")) {
      const [t, i] = Ms(this.item);
      this.style.setProperty("--left-symbol-bg-image-url", t), this.style.setProperty("--right-symbol-bg-image-url", i);
    }
  }
  render() {
    const e = Le(this.item.frameType), t = e ? si(e, this.item.identified) : null;
    return u`<header
			class=${lt({
      header: !0,
      "header--single": t === "single",
      "header--double": t === "double",
      "header--necropolis": e === "necropolis",
      [`${e}`]: !0
    })}
			style="background: ${Rs(e ?? "normal", this.item.identified)}"
		>
			<div class="symbol left-symbol"></div>
			<div class="symbol right-symbol"></div>
			${t === "double" ? u`
						<div class="content mt-2">${this.item.name}</div>
						<div class="content mb-4">${this.item.baseType}</div>
				  ` : u`<div class="content">${this.item.baseType}</div>`}
		</header>`;
  }
};
St.styles = S`
		${Os}

		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}

		:host {
			--left-symbol-bg-image-url: none;
			--right-symbol-bg-image-url: none;
			${Ps}
		}

		.header {
			font-family: 'fontin';
			display: flex;
			justify-content: center;
			align-items: center;
			flex-direction: column;
			position: relative;
			height: var(--height);
		}

		.header--single {
			--height: 33px;
		}

		.header--double {
			--height: 54px;
		}

		.header--necropolis {
			--height: 45px;
		}

		.content {
			padding-inline: 2.5rem;
			font-size: 19px;
			display: flex;
			align-items: center;
			justify-content: center;
			height: 100%;
		}

		.mt-2 {
			margin-top: 2px;
		}

		.mb-4 {
			margin-bottom: 0.25rem;
		}

		.symbol {
			width: calc(var(--cell-size) / var(--default-cell-size) * 27);
			height: var(--height, 33px);
			background-size: 100%;
			width: 27px;
		}
		.left-symbol {
			background: var(--left-symbol-bg-image-url) center no-repeat;
			position: absolute;
			left: 0px;
			z-index: 20000000000;
		}
		.right-symbol {
			background: var(--right-symbol-bg-image-url) center no-repeat;
			position: absolute;
			right: 0px;
			z-index: 20000000000;
		}
	`;
ii([
  $({ type: Object })
], St.prototype, "item", 2);
St = ii([
  P("poe-item-info-header")
], St);
function Ms(e) {
  return (() => {
    if (e.influences) {
      const i = Object.keys(e.influences);
      switch (i.length) {
        case 0:
          return ["", ""];
        case 1:
          return [i[0], i[0]];
        case 2:
          return [i[0], i[1]];
      }
    }
    return e.fractured ? ["fractured", "fractured"] : e.synthesised ? ["synthesised", "synthesised"] : ["", ""];
  })().map((i) => `url(/poe-images/${i}-symbol.png)`);
}
function Rs(e, t) {
  const i = si(e, t), s = `url(${Nt(e, i, "left")}) top left no-repeat`, n = `url(${Nt(e, i, "right")}) top right no-repeat`, r = `url(${Nt(e, i, "middle")}) top center`;
  return `${s},${n},${r}`;
}
function Nt(e, t, i, s = "/poe-images/") {
  return `${s}${["header", t === "double" ? "double" : "", e, i].filter((n) => n.length > 0).join("-")}.png`;
}
function si(e, t) {
  switch (e) {
    case "normal":
      return "single";
    case "magic":
      return "single";
    case "rare":
      return t ? "double" : "single";
    case "unique":
      return t ? "double" : "single";
    case "gem":
      return "single";
    case "currency":
      return "single";
    case "necropolis":
      return "single";
  }
}
var Ls = Object.defineProperty, js = Object.getOwnPropertyDescriptor, ni = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? js(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && Ls(t, i, n), n;
};
let Et = class extends _ {
  render() {
    return this.item ? u`<poe-item-info-header .item=${this.item}></poe-item-info-header>
			<poe-item-info-content .item=${this.item}></poe-item-info-content> ` : u`<p style="color: red">No Poe Api item data (.item)</p>`;
  }
};
Et.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			display: block;
			width: fit-content;
			min-width: max-content;
		}

		poe-item-info-content {
			padding-inline: 1rem;
		}
	`;
ni([
  $({ type: Object })
], Et.prototype, "item", 2);
Et = ni([
  P("poe-item-info")
], Et);
var zs = Object.defineProperty, Ds = Object.getOwnPropertyDescriptor, I = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? Ds(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && zs(t, i, n), n;
}, ri = (e, t, i) => {
  if (!t.has(e))
    throw TypeError("Cannot " + i);
}, xe = (e, t, i) => (ri(e, t, "read from private field"), i ? i.call(e) : t.get(e)), Us = (e, t, i) => {
  if (t.has(e))
    throw TypeError("Cannot add the same private member more than once");
  t instanceof WeakSet ? t.add(e) : t.set(e, i);
}, Ns = (e, t, i, s) => (ri(e, t, "write to private field"), s ? s.call(e, i) : t.set(e, i), i), et;
let M = class extends _ {
  constructor() {
    super(), Us(this, et, null), this.showSockets = !1, this.placed = !1, this.socketsVisible = !1, this.hovered = !1, this.altPressed = !1, this.onJClick = (e) => {
      if (this.hovered && e.code === "KeyJ") {
        const t = this.iconJson ?? document.createElement("tooltip-json-icon");
        this.iconJson || this.shadowRoot.append(t), navigator.clipboard.writeText(JSON.stringify(this.item, null, 4)), t.showing = !0, setTimeout(() => {
          t.showing = !1;
        }, 2e3);
      }
    }, this.onHoverCtrlCClick = (e) => {
      this.hovered && e.ctrlKey && e.code === "KeyC" && (console.log("ctrl c clicked!"), xe(this, et) || Ns(this, et, new Is(this.item)), window.navigator.clipboard.writeText(xe(this, et).transform()));
    }, this.onAltPressed = (e) => {
      e.key === "Alt" && (e.preventDefault(), this.altPressed = !0);
    }, this.onAltReleased = () => {
      this.altPressed = !1;
    }, this.addEventListener("mouseenter", this.onMouseEnter), this.addEventListener("mouseleave", this.onMouseLeave);
  }
  async willUpdate(e) {
    e.has("item") && (this.style.setProperty("--w", this.item.w.toString()), this.style.setProperty("--h", this.item.h.toString()), this.item.identified || this.setAttribute("unidentified", ""), this.style.setProperty("--influence-background-image-url", Hs(this.item))), e.has("showSockets") && (this.socketsVisible = this.showSockets), e.has("altPressed") && (this.altPressed ? this.socketsVisible = !0 : this.socketsVisible = this.showSockets), e.has("hovered") && (this.hovered ? this.socketsVisible = !0 : this.socketsVisible = this.showSockets);
  }
  get tooltipElement() {
    const e = this.nextElementSibling;
    return e instanceof F ? e : null;
  }
  render() {
    return this.item ? u`
			<img alt=${this.item.baseType} .src=${this.item.icon} />
			${this.item.socketedItems && this.item.sockets ? u`<poe-socket-chain
						@hovered-socketed-item-changed=${this.onHoveredSocketedItemChanged}
						class=${lt({ hidden: !this.socketsVisible })}
						.socketedItems=${this.item.socketedItems}
						.sockets=${this.item.sockets}
						.w=${this.item.w}
				  ></poe-socket-chain>` : f}
			${this.item.stackSize ? u`<p class="stackSize">${this.item.stackSizeText || this.item.stackSize}</p>` : f}
		` : u`<p style="color: red">No Poe Api item data (.item)</p>`;
  }
  onHoveredSocketedItemChanged(e) {
    if (this.tooltipElement) {
      const t = this.tooltipElement.querySelector(".socketed-item");
      if (t instanceof HTMLElement) {
        t.innerHTML = "";
        const i = document.createElement("poe-item-info");
        e.detail && (i.item = e.detail, t.append(i));
      }
    }
  }
  firstUpdated() {
    F.lazy(this, (e) => {
      Ce(
        u`<div
					style="display:flex;align-items:flex-start;flex-wrap:wrap;gap:1.2rem;z-index:500;padding:0;margin:0"
				>
					<poe-item-info .item=${this.item}></poe-item-info>
					<div class="socketed-item"></div>
				</div>`,
        e
      );
    });
  }
  onMouseEnter() {
    this.hovered = !0;
  }
  onMouseLeave() {
    this.hovered = !1;
  }
  connectedCallback() {
    super.connectedCallback(), De(), window.addEventListener("keydown", this.onAltPressed), window.addEventListener("keyup", this.onAltReleased), window.addEventListener("keydown", this.onJClick), window.addEventListener("keydown", this.onHoverCtrlCClick);
  }
  disconnectedCallback() {
    super.disconnectedCallback(), window.removeEventListener("keydown", this.onAltPressed), window.removeEventListener("keyup", this.onAltReleased), window.removeEventListener("keydown", this.onJClick), window.removeEventListener("keydown", this.onHoverCtrlCClick);
  }
};
et = /* @__PURE__ */ new WeakMap();
M.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			--influence-background-image-url: none;
			--background-color: none;
			--default-cell-size: 47;
			--cell-size: 47px; /** css prop */
			--w: '(computed) number of horizontal cells';
			--h: '(computed) number of vertical cells';
			width: calc(var(--cell-size) * var(--w));
			height: calc(var(--cell-size) * var(--h));
			background: var(--influence-background-image-url);
			background-color: var(--background-color);

			position: relative;
			display: flex;
			justify-content: center;
			align-items: center;
			font-family: fontin;
		}

		:host([placed]) {
			background-color: rgba(25, 26, 150, 0.25);
		}

		:host([unidentified]) {
			background-color: rgba(210, 0, 0, 0.18) !important;
		}

		.socketed-item {
			display: flex;
			flex-wrap: wrap;
			gap: 2000px;
		}

		.stackSize {
			font-size: calc(var(--cell-size) / var(--default-cell-size) * 18);
			font-weight: bold;
			color: #fff;
			position: absolute;
			top: -1px;
			left: 5%;
			text-shadow: -1px -1px 0 #000, 0 -1px 0 #000, 1px -1px 0 #000, 1px 0 0 #000, 1px 1px 0 #000, 0 1px 0 #000,
				-1px 1px 0 #000, -1px 0 0 #000, -1px -1px 3px #000, 0 -1px 3px #000, 1px -1px 0 #000, 1px 0 3px #000,
				1px 1px 3px #000, 0 1px 3px #000, -1px 1px 3px #000, -1px 0 3px #000;
			pointer-events: none;
		}

		img {
			display: block;
			width: 100%;
		}

		poe-socket-chain {
			position: absolute;
		}

		.hidden {
			display: none !important;
		}
	`;
I([
  $({ type: Object })
], M.prototype, "item", 2);
I([
  $({ type: Boolean, reflect: !0, attribute: "show-sockets" })
], M.prototype, "showSockets", 2);
I([
  $({ type: Boolean })
], M.prototype, "placed", 2);
I([
  Ot()
], M.prototype, "socketsVisible", 2);
I([
  Ot()
], M.prototype, "hovered", 2);
I([
  Ot()
], M.prototype, "altPressed", 2);
I([
  Ci("tooltip-json-icon")
], M.prototype, "iconJson", 2);
M = I([
  P("poe-item")
], M);
function Hs(e) {
  if (!e.influences)
    return "";
  const t = Object.keys(e.influences), i = (s) => {
    switch (s) {
      case "shaper":
      case "elder":
        return `url(/poe-images/${je(s)}Backgroundw${e.w}h${e.h}.png) no-repeat`;
      default:
        return "";
    }
  };
  return t.map(i).filter(Boolean).join(", ");
}
var kt;
class Is {
  constructor(t) {
    ee(this, kt, `
--------
`);
    this.item = t;
  }
  transform() {
    return [
      [
        this.item.rarity ? `Rarity: ${this.item.rarity}` : "",
        `${this.item.name === this.item.baseType}` ? "" : this.item.name,
        this.item.baseType
      ].filter((t) => t.length > 0).join(`
`),
      this.properties.length ? this.properties.map(Bs).join(`
`) : "",
      this.requirements.length ? `Requirements: 
${this.requirements.map(({ name: t, values: i }) => `${t}: ${i[0][0]}`).join(`
`)}` : "",
      this.sockets.length ? `Sockets: ${Object.values(Object.groupBy(this.sockets, (t) => t.group)).flatMap((t = []) => t.map(({ sColour: i }) => i).join("-")).join(" ")}` : "",
      this.enchantments.length ? this.enchantments.join(`
`) : "",
      this.implicits.length ? this.implicits.join(`
`) : "",
      this.fracturedMods.length || this.explicits.length || this.crafts.length ? [...this.fracturedMods, ...this.explicits, ...this.crafts].join(`
`) : "",
      this.item.identified ? "" : "Unidentified"
    ].filter((t) => t.length > 0).flatMap((t, i, s) => i === s.length - 1 ? [t] : [t, te(this, kt)]).join("");
  }
  groupSockets() {
    Object.values(Object.groupBy(this.sockets, (t) => t.group)).flatMap((t = []) => t.map(({ sColour: i }) => i).join("-")).map((t) => (console.log(t), t)).join(" ");
  }
  get sockets() {
    return this.item.sockets ?? [];
  }
  get enchantments() {
    return this.item.enchantMods ?? [];
  }
  get properties() {
    return this.item.properties ?? [];
  }
  get requirements() {
    return this.item.requirements ?? [];
  }
  get implicits() {
    return this.item.implicitMods ?? [];
  }
  get explicits() {
    return this.item.explicitMods ?? [];
  }
  get crafts() {
    return this.item.craftedMods ?? [];
  }
  get fracturedMods() {
    return this.item.fracturedMods ?? [];
  }
}
kt = new WeakMap();
function Bs(e) {
  switch (e.displayMode) {
    case 0:
      return e.values.length ? `${e.name}: ${e.values.map((t) => t[0]).join(", ")}` : e.name;
    case 3:
      return ze(e);
    default:
      return "";
  }
}
/**
 * @license
 * Copyright 2018 Google LLC
 * SPDX-License-Identifier: BSD-3-Clause
 */
const oi = "important", Vs = " !" + oi, qs = Me(class extends Re {
  constructor(e) {
    var t;
    if (super(e), e.type !== Te.ATTRIBUTE || e.name !== "style" || ((t = e.strings) == null ? void 0 : t.length) > 2)
      throw Error("The `styleMap` directive must be used in the `style` attribute and must be the only part in the attribute.");
  }
  render(e) {
    return Object.keys(e).reduce((t, i) => {
      const s = e[i];
      return s == null ? t : t + `${i = i.includes("-") ? i : i.replace(/(?:^(webkit|moz|ms|o)|)(?=[A-Z])/g, "-$&").toLowerCase()}:${s};`;
    }, "");
  }
  update(e, [t]) {
    const { style: i } = e.element;
    if (this.ft === void 0)
      return this.ft = new Set(Object.keys(t)), this.render(t);
    for (const s of this.ft)
      t[s] == null && (this.ft.delete(s), s.includes("-") ? i.removeProperty(s) : i[s] = null);
    for (const s in t) {
      const n = t[s];
      if (n != null) {
        this.ft.add(s);
        const r = typeof n == "string" && n.endsWith(Vs);
        s.includes("-") || r ? i.setProperty(s, r ? n.slice(0, -11) : n, r ? oi : "") : i[s] = n;
      }
    }
    return D;
  }
});
var Fs = Object.defineProperty, Ws = Object.getOwnPropertyDescriptor, Gt = (e, t, i, s) => {
  for (var n = s > 1 ? void 0 : s ? Ws(t, i) : t, r = e.length - 1, o; r >= 0; r--)
    (o = e[r]) && (n = (s ? o(t, i, n) : o(n)) || n);
  return s && n && Fs(t, i, n), n;
};
const Ys = [
  "NormalStash",
  "PremiumStash",
  "QuadStash",
  "EssenceStash",
  "CurrencyStash",
  "FragmentStash",
  "BlightStash",
  "DivinationCardStash"
];
let ht = class extends _ {
  constructor() {
    super(...arguments), this.onKeyDown = async (e) => {
      if (this.focusWithin && ["ArrowDown", "ArrowRight", "ArrowUp", "ArrowLeft"].includes(e.code)) {
        const t = e.code.slice(5).toLowerCase(), i = this.activeItemElement;
        if (i) {
          const s = await Qs({
            activeItem: i.item,
            direction: t,
            items: this.tabState.items,
            tabElement: this,
            tabCellsSideCount: Bt(this.tabState.type)
          });
          s && s.focus();
        }
      }
    };
  }
  get focusWithin() {
    return this.matches(":focus-within");
  }
  get activeItemElement() {
    var e;
    return ((e = this.shadowRoot) == null ? void 0 : e.querySelector("poe-item:focus")) ?? null;
  }
  willUpdate(e) {
    if (e.has("tab")) {
      this.tabState = structuredClone(this.tab);
      const t = Bt(this.tabState.type);
      Xs(this.tabState, t), this.tabState.items = Ks(this.tabState.items), this.style.setProperty("--cells-side-count", t.toString()), this.style.setProperty("--background-image", `url(${Js(this.tabState.type)})`);
    }
  }
  render() {
    return this.tab ? Ys.includes(this.tab.type) ? u`
			<ul>
				${this.tabState.items.map(
      (e) => u`<li
						style=${qs({
        "grid-column": `${e.x + 1} / span ${e.w}`,
        "grid-row": `${e.y + 1} / span ${e.h}`
      })}
					>
						<poe-item
							data-x=${e.x}
							data-y=${e.y}
							tabindex="0"
							placed
							style="--cell-size: ${Gs(this.tabState.type)}"
							.item=${e}
						></poe-item>
					</li>`
    )}
			</ul>
		` : (this.style.setProperty("border", "2px solid red"), u`<p style="color: red; font-size: 24px">
				StashType ( ${this.tab.type} ) is not supported ( yet? ).
			</p>`) : (this.style.setProperty("border", "2px solid red"), u`<p style="color: red">No Poe Api stash tab data (.tab)</p>`);
  }
  connectedCallback() {
    super.connectedCallback(), De(), window.addEventListener("keydown", this.onKeyDown);
  }
  disconnectedCallback() {
    super.disconnectedCallback(), window.removeEventListener("keydown", this.onKeyDown);
  }
};
ht.styles = S`
		* {
			padding: 0;
			margin: 0;
			box-sizing: border-box;
		}
		:host {
			display: block;
			--size: 569px;
			--size-of-all-inner-borders: 5px;
			--cells-side-count: '(computed) Number of cells';
			--background-image: '(computed)';
			width: var(--size);
			height: var(--size);
			background-image: var(--background-image);
			font-family: fontin;
		}

		ul {
			width: var(--size);
			height: var(--size);
			list-style: none;
			display: grid;
			grid-template-rows: repeat(var(--cells-side-count), 1fr);
			grid-template-columns: repeat(var(--cells-side-count), 1fr);
			gap: calc(var(--size-of-all-inner-borders) / var(--cells-side-count));
		}

		poe-item:focus,
		poe-item:focus-visible {
			outline: 3px solid rgb(39, 186, 253);
		}
		:host(:focus-within) {
			outline: 3px solid rgb(39, 186, 253);
		}
	`;
Gt([
  $({ type: Object })
], ht.prototype, "tab", 2);
Gt([
  Ot()
], ht.prototype, "tabState", 2);
ht = Gt([
  P("poe-stash-tab")
], ht);
function Xs(e, t) {
  if (t) {
    if (e.type === "FragmentStash") {
      let i = 0;
      const s = 12, n = 13;
      e.items.forEach((r) => {
        switch (r.y) {
          case 0: {
            r.y = Math.floor(r.x / t), r.x = r.x % t;
            break;
          }
          case 1: {
            r.y = s + Math.floor(i / t), r.x = i % t, i++;
            break;
          }
          case 2: {
            r.y += n;
            break;
          }
          default:
            console.warn(`Fragments stash unexpected item Y-coordinate. Expected 0|1|2, got ${r.y}`);
        }
      });
    }
    (e.type === "EssenceStash" || e.type === "CurrencyStash" || e.type === "BlightStash" || e.type === "DivinationCardStash") && e.items.forEach((i) => {
      i.y = Math.floor(i.x / t), i.x = i.x % t;
    });
  }
}
function Bt(e) {
  switch (e) {
    case "PremiumStash":
    case "NormalStash":
    case "BlightStash":
      return 12;
    case "QuadStash":
    case "FragmentStash":
    case "DivinationCardStash":
      return 24;
    default:
      return 12;
  }
}
function Js(e) {
  switch (e) {
    case "PremiumStash":
    case "NormalStash":
    case "EssenceStash":
    case "CurrencyStash":
    case "BlightStash":
      return "/poe-images/StashPanelGrid.png";
    case "QuadStash":
    case "FragmentStash":
    case "DivinationCardStash":
      return "/poe-images/QuadStashPanelGrid.png";
    default:
      return "/poe-images/StashPanelGrid.png";
  }
}
function Gs(e) {
  return `${564 / Bt(e)}px`;
}
function Ks(e) {
  return Object.values(Object.groupBy(e, ({ y: t }) => t)).flatMap((t = []) => (t.sort((i, s) => i.x - s.x), t));
}
async function Qs({
  activeItem: e,
  direction: t,
  items: i,
  tabElement: s,
  tabCellsSideCount: n
}) {
  if (t === "down") {
    let r = e.x, o = e.y + e.h, a = null;
    if (o + 1 > n)
      return null;
    for (; !a; ) {
      for (let l = 0; l < e.w && (a = await ft({
        tabElement: s,
        items: i,
        coordinates: { x: r + l, y: o }
      }), !a); l++)
        ;
      if (o === n - 1 && r === n - 1)
        break;
      o === n - 1 ? (o = e.y, r++) : o++;
    }
    return a;
  } else if (t === "right") {
    let r = e.x + e.w, o = e.y, a = null;
    if (r + 1 > n)
      return null;
    for (; !a; ) {
      for (let l = 0; l < e.h && (a = await ft({
        tabElement: s,
        items: i,
        coordinates: { x: r, y: o + l }
      }), !a); l++)
        ;
      if (o === n - 1 && r === n - 1)
        break;
      r === n - 1 ? (r = e.x + e.w, o++) : r++;
    }
    return a;
  } else if (t === "up") {
    let r = e.x, o = e.y - 1, a = null;
    if (o < 0)
      return null;
    for (; !a; ) {
      for (let l = 0; l < e.w && (a = await ft({
        tabElement: s,
        items: i,
        coordinates: { x: r + l, y: o }
      }), !a); l++)
        ;
      if (o === 0 && r === n - 1)
        break;
      o === 0 ? (o = e.y, r = r + 1) : o--;
    }
    return a;
  } else if (t === "left") {
    let r = e.x - 1, o = e.y, a = null;
    if (r < 0)
      return null;
    for (; !a; ) {
      for (let l = 0; l < e.h && (a = await ft({
        tabElement: s,
        items: i,
        coordinates: { x: r, y: o + l }
      }), !a); l++)
        ;
      if (r === 0 && o === n - 1)
        break;
      r === 0 ? (r = e.x, o = o + 1) : r--;
    }
    return a;
  }
}
function Zs({
  coordinates: e,
  items: t
}) {
  const i = t.find((s) => s.x === e.x && s.y === e.y);
  if (i)
    return i;
  for (const s of t)
    for (let n = s.y; n < s.y + s.h; n++)
      for (let r = s.x; r < s.x + s.w; r++)
        if (r === e.x && n === e.y)
          return s;
  return null;
}
async function ft({
  tabElement: e,
  coordinates: t,
  items: i
}) {
  var n;
  const s = Zs({
    coordinates: t,
    items: i
  });
  return s ? (e.shadowRoot.querySelector("poe-item") || await e.updateComplete, ((n = e.shadowRoot) == null ? void 0 : n.querySelector(`poe-item[data-x="${s.x}"][data-y="${s.y}"]`)) ?? null) : null;
}
export {
  ht as PoeStashTabElement
};
