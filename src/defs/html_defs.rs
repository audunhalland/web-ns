pub mod attrs {
    // This import path is altered because this file is included from build.rs
    use crate::attr_type::flags::*;

    pub const DEFS: &[(&str, &str, u32)] = &[
        ("abbr", "abbr", STRING),
        ("accept", "accept", COMMA_SEP | STRING),
        ("accept-charset", "acceptCharset", SPACE_SEP | STRING),
        ("accesskey", "accessKey", SPACE_SEP | STRING),
        ("action", "action", STRING),
        ("allow", "allow", STRING),
        ("allowfullscreen", "allowFullScreen", BOOL),
        ("allowpaymentrequest", "allowPaymentRequest", BOOL),
        ("allowusermedia", "allowUserMedia", BOOL),
        ("alt", "alt", STRING),
        ("as", "as", STRING),
        ("async", "async", BOOL),
        ("autocapitalize", "autoCapitalize", STRING),
        ("autocomplete", "autoComplete", SPACE_SEP | STRING),
        ("autofocus", "autoFocus", BOOL),
        ("autoplay", "autoPlay", BOOL),
        ("capture", "capture", BOOL),
        ("charset", "charSet", STRING),
        ("checked", "checked", BOOL),
        ("cite", "cite", STRING),
        ("class", "className", SPACE_SEP | STRING),
        ("cols", "cols", NUMBER),
        ("colspan", "colSpan", STRING),
        ("content", "content", STRING),
        (
            "contenteditable",
            "contentEditable",
            TRUE | EMPTY_STRING | FALSE,
        ),
        ("controls", "controls", BOOL),
        ("controlslist", "controlsList", SPACE_SEP | STRING),
        ("coords", "coords", COMMA_SEP | STRING),
        ("crossorigin", "crossOrigin", STRING),
        ("data", "data", STRING),
        ("datetime", "dateTime", STRING),
        ("decoding", "decoding", STRING),
        ("default", "default", BOOL),
        ("defer", "defer", BOOL),
        ("dir", "dir", STRING),
        ("dirname", "dirName", STRING),
        ("disabled", "disabled", BOOL),
        ("download", "download", BOOL | STRING),
        ("draggable", "draggable", TRUE | FALSE),
        ("enctype", "encType", STRING),
        ("enterkeyhint", "enterKeyHint", STRING),
        ("form", "form", STRING),
        ("formaction", "formAction", STRING),
        ("formenctype", "formEncType", STRING),
        ("formmethod", "formMethod", STRING),
        ("formnovalidate", "formNoValidate", BOOL),
        ("formtarget", "formTarget", STRING),
        ("headers", "headers", SPACE_SEP | STRING),
        ("height", "height", NUMBER),
        ("hidden", "hidden", BOOL),
        ("high", "high", NUMBER),
        ("href", "href", STRING),
        ("hreflang", "hrefLang", STRING),
        ("for", "htmlFor", SPACE_SEP | STRING),
        ("http-equiv", "httpEquiv", SPACE_SEP | STRING),
        ("id", "id", STRING),
        ("imagesizes", "imageSizes", STRING),
        ("imagesrcset", "imageSrcSet", COMMA_SEP | STRING),
        ("inputmode", "inputMode", STRING),
        ("integrity", "integrity", STRING),
        ("is", "is", STRING),
        ("ismap", "isMap", BOOL),
        ("itemid", "itemId", STRING),
        ("itemprop", "itemProp", SPACE_SEP | STRING),
        ("itemref", "itemRef", SPACE_SEP | STRING),
        ("itemscope", "itemScope", BOOL),
        ("itemtype", "itemType", SPACE_SEP | STRING),
        ("kind", "kind", STRING),
        ("label", "label", STRING),
        ("lang", "lang", STRING),
        ("language", "language", STRING),
        ("list", "list", STRING),
        ("loading", "loading", STRING),
        ("loop", "loop", BOOL),
        ("low", "low", NUMBER),
        ("manifest", "manifest", STRING),
        ("max", "max", STRING),
        ("maxlength", "maxLength", NUMBER),
        ("media", "media", STRING),
        ("method", "method", STRING),
        ("min", "min", STRING),
        ("minlength", "minLength", NUMBER),
        ("multiple", "multiple", BOOL),
        ("muted", "muted", BOOL),
        ("name", "name", STRING),
        ("nonce", "nonce", STRING),
        ("nomodule", "noModule", BOOL),
        ("novalidate", "noValidate", BOOL),
        ("onabort", "onAbort", STRING),
        ("onafterprint", "onAfterPrint", STRING),
        ("onauxclick", "onAuxClick", STRING),
        ("onbeforeprint", "onBeforePrint", STRING),
        ("onbeforeunload", "onBeforeUnload", STRING),
        ("onblur", "onBlur", STRING),
        ("oncancel", "onCancel", STRING),
        ("oncanplay", "onCanPlay", STRING),
        ("oncanplaythrough", "onCanPlayThrough", STRING),
        ("onchange", "onChange", STRING),
        ("onclick", "onClick", STRING),
        ("onclose", "onClose", STRING),
        ("oncontextmenu", "onContextMenu", STRING),
        ("oncopy", "onCopy", STRING),
        ("oncuechange", "onCueChange", STRING),
        ("oncut", "onCut", STRING),
        ("ondblclick", "onDblClick", STRING),
        ("ondrag", "onDrag", STRING),
        ("ondragend", "onDragEnd", STRING),
        ("ondragenter", "onDragEnter", STRING),
        ("ondragexit", "onDragExit", STRING),
        ("ondragleave", "onDragLeave", STRING),
        ("ondragover", "onDragOver", STRING),
        ("ondragstart", "onDragStart", STRING),
        ("ondrop", "onDrop", STRING),
        ("ondurationchange", "onDurationChange", STRING),
        ("onemptied", "onEmptied", STRING),
        ("onended", "onEnded", STRING),
        ("onerror", "onError", STRING),
        ("onfocus", "onFocus", STRING),
        ("onformdata", "onFormData", STRING),
        ("onhashchange", "onHashChange", STRING),
        ("oninput", "onInput", STRING),
        ("oninvalid", "onInvalid", STRING),
        ("onkeydown", "onKeyDown", STRING),
        ("onkeypress", "onKeyPress", STRING),
        ("onkeyup", "onKeyUp", STRING),
        ("onlanguagechange", "onLanguageChange", STRING),
        ("onload", "onLoad", STRING),
        ("onloadeddata", "onLoadedData", STRING),
        ("onloadedmetadata", "onLoadedMetadata", STRING),
        ("onloadend", "onLoadEnd", STRING),
        ("onloadstart", "onLoadStart", STRING),
        ("onmessage", "onMessage", STRING),
        ("onmessageerror", "onMessageError", STRING),
        ("onmousedown", "onMouseDown", STRING),
        ("onmouseenter", "onMouseEnter", STRING),
        ("onmouseleave", "onMouseLeave", STRING),
        ("onmousemove", "onMouseMove", STRING),
        ("onmouseout", "onMouseOut", STRING),
        ("onmouseover", "onMouseOver", STRING),
        ("onmouseup", "onMouseUp", STRING),
        ("onoffline", "onOffline", STRING),
        ("ononline", "onOnline", STRING),
        ("onpagehide", "onPageHide", STRING),
        ("onpageshow", "onPageShow", STRING),
        ("onpaste", "onPaste", STRING),
        ("onpause", "onPause", STRING),
        ("onplay", "onPlay", STRING),
        ("onplaying", "onPlaying", STRING),
        ("onpopstate", "onPopState", STRING),
        ("onprogress", "onProgress", STRING),
        ("onratechange", "onRateChange", STRING),
        ("onrejectionhandled", "onRejectionHandled", STRING),
        ("onreset", "onReset", STRING),
        ("onresize", "onResize", STRING),
        ("onscroll", "onScroll", STRING),
        (
            "onsecuritypolicyviolation",
            "onSecurityPolicyViolation",
            STRING,
        ),
        ("onseeked", "onSeeked", STRING),
        ("onseeking", "onSeeking", STRING),
        ("onselect", "onSelect", STRING),
        ("onslotchange", "onSlotChange", STRING),
        ("onstalled", "onStalled", STRING),
        ("onstorage", "onStorage", STRING),
        ("onsubmit", "onSubmit", STRING),
        ("onsuspend", "onSuspend", STRING),
        ("ontimeupdate", "onTimeUpdate", STRING),
        ("ontoggle", "onToggle", STRING),
        ("onunhandledrejection", "onUnhandledRejection", STRING),
        ("onunload", "onUnload", STRING),
        ("onvolumechange", "onVolumeChange", STRING),
        ("onwaiting", "onWaiting", STRING),
        ("onwheel", "onWheel", STRING),
        ("open", "open", BOOL),
        ("optimum", "optimum", NUMBER),
        ("pattern", "pattern", STRING),
        ("ping", "ping", SPACE_SEP | STRING),
        ("placeholder", "placeholder", STRING),
        ("playsinline", "playsInline", BOOL),
        ("poster", "poster", STRING),
        ("preload", "preload", STRING),
        ("readonly", "readOnly", BOOL),
        ("referrerpolicy", "referrerPolicy", STRING),
        ("rel", "rel", SPACE_SEP | STRING),
        ("required", "required", BOOL),
        ("reversed", "reversed", BOOL),
        ("rows", "rows", NUMBER),
        ("rowspan", "rowSpan", NUMBER),
        ("sandbox", "sandbox", SPACE_SEP | STRING),
        ("scope", "scope", STRING),
        ("scoped", "scoped", BOOL),
        ("seamless", "seamless", BOOL),
        ("selected", "selected", BOOL),
        ("shape", "shape", STRING),
        ("size", "size", NUMBER),
        ("sizes", "sizes", STRING),
        ("slot", "slot", STRING),
        ("span", "span", NUMBER),
        ("spellcheck", "spellCheck", TRUE | FALSE),
        ("src", "src", STRING),
        ("srcdoc", "srcDoc", STRING),
        ("srclang", "srcLang", STRING),
        ("srcset", "srcSet", COMMA_SEP | STRING),
        ("start", "start", NUMBER),
        ("step", "step", STRING),
        ("style", "style", STRING),
        ("tabindex", "tabIndex", NUMBER),
        ("target", "target", STRING),
        ("title", "title", STRING),
        ("translate", "translate", STRING),
        ("type", "type", STRING),
        ("typemustmatch", "typeMustMatch", BOOL),
        ("usemap", "useMap", STRING),
        ("value", "value", TRUE | FALSE | STRING),
        ("width", "width", NUMBER),
        ("wrap", "wrap", STRING),
        // Legacy.
        // See: https://html.spec.whatwg.org/#other-elements,-attributes-and-apis
        ("align", "align", STRING), // Several. Use CSS `text-align` instead,
        ("alink", "aLink", STRING), // `<body>`. Use CSS `a:active {color}` instead
        ("archive", "archive", SPACE_SEP | STRING), // `<object>`. List of URIs to archives
        ("axis", "axis", STRING),   // `<td>` and `<th>`. Use `scope` on `<th>`
        ("background", "background", STRING), // `<body>`. Use CSS `background-image` instead
        ("bgcolor", "bgColor", STRING), // `<body>` and table elements. Use CSS `background-color` instead
        ("border", "border", NUMBER),   // `<table>`. Use CSS `border-width` instead,
        ("bordercolor", "borderColor", STRING), // `<table>`. Use CSS `border-color` instead,
        ("bottommargin", "bottomMargin", NUMBER), // `<body>`
        ("cellpadding", "cellPadding", STRING), // `<table>`
        ("cellspacing", "cellSpacing", STRING), // `<table>`
        ("char", "char", STRING), // Several table elements. When `align=char`, sets the character to align on
        ("charoff", "charOff", STRING), // Several table elements. When `char`, offsets the alignment
        ("classid", "classId", STRING), // `<object>`
        ("clear", "clear", STRING),     // `<br>`. Use CSS `clear` instead
        ("code", "code", STRING),       // `<object>`
        ("codebase", "codeBase", STRING), // `<object>`
        ("codetype", "codeType", STRING), // `<object>`
        ("color", "color", STRING),     // `<font>` and `<hr>`. Use CSS instead
        ("compact", "compact", BOOL),   // Lists. Use CSS to reduce space between items instead
        ("declare", "declare", BOOL),   // `<object>`
        ("event", "event", STRING),     // `<script>`
        ("face", "face", STRING),       // `<font>`. Use CSS instead
        ("frame", "frame", STRING),     // `<table>`
        ("frameborder", "frameBorder", STRING), // `<iframe>`. Use CSS `border` instead
        ("hspace", "hSpace", NUMBER),   // `<img>` and `<object>`
        ("leftmargin", "leftMargin", NUMBER), // `<body>`
        ("link", "link", STRING),       // `<body>`. Use CSS `a:link {color: *}` instead
        ("longdesc", "longDesc", STRING), // `<frame>`, `<iframe>`, and `<img>`. Use an `<a>`
        ("lowsrc", "lowSrc", STRING),   // `<img>`. Use a `<picture>`
        ("marginheight", "marginHeight", NUMBER), // `<body>`
        ("marginwidth", "marginWidth", NUMBER), // `<body>`
        ("noresize", "noResize", BOOL), // `<frame>`
        ("nohref", "noHref", BOOL),     // `<area>`. Use no href instead of an explicit `nohref`
        ("noshade", "noShade", BOOL), // `<hr>`. Use background-color and height instead of borders
        ("nowrap", "noWrap", BOOL),   // `<td>` and `<th>`
        ("object", "object", STRING), // `<applet>`
        ("profile", "profile", STRING), // `<head>`
        ("prompt", "prompt", STRING), // `<isindex>`
        ("rev", "rev", STRING),       // `<link>`
        ("rightmargin", "rightMargin", NUMBER), // `<body>`
        ("rules", "rules", STRING),   // `<table>`
        ("scheme", "scheme", STRING), // `<meta>`
        ("scrolling", "scrolling", TRUE | FALSE | STRING), // `<frame>`. Use overflow in the child context
        ("standby", "standby", STRING),                    // `<object>`
        ("summary", "summary", STRING),                    // `<table>`
        ("text", "text", STRING),                          // `<body>`. Use CSS `color` instead
        ("topmargin", "topMargin", NUMBER),                // `<body>`
        ("valuetype", "valueType", STRING),                // `<param>`
        ("version", "version", STRING),                    // `<html>`. Use a doctype.
        ("valign", "vAlign", STRING), // Several. Use CSS `vertical-align` instead
        ("vlink", "vLink", STRING),   // `<body>`. Use CSS `a:visited {color}` instead
        ("vspace", "vSpace", NUMBER), // `<img>` and `<object>`
        // Non-standard Properties.
        ("allowtransparency", "allowTransparency", STRING),
        ("autocorrect", "autoCorrect", STRING),
        ("autosave", "autoSave", STRING),
        ("disablepictureinpicture", "disablePictureInPicture", BOOL),
        ("disableremoteplayback", "disableRemotePlayback", BOOL),
        ("prefix", "prefix", STRING),
        ("property", "property", STRING),
        ("results", "results", NUMBER),
        ("security", "security", STRING),
        ("unselectable", "unselectable", STRING),
    ];
}
