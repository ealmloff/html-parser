#![allow(clippy::enum_variant_names)]
#![allow(dead_code)]
use kalosm_sample::*;
#[derive(Debug, Clone, Copy, Parse)]
pub enum BValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum UValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "true")]
    True,
    #[parse(rename = "undefined")]
    Undefined,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum OValues {
    #[parse(rename = "off")]
    Off,
    #[parse(rename = "on")]
    On,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum YValues {
    #[parse(rename = "no")]
    No,
    #[parse(rename = "yes")]
    Yes,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum WValues {
    #[parse(rename = "hard")]
    Hard,
    #[parse(rename = "soft")]
    Soft,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum DValues {
    #[parse(rename = "auto")]
    Auto,
    #[parse(rename = "ltr")]
    Ltr,
    #[parse(rename = "rtl")]
    Rtl,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MValues {
    #[parse(rename = "dialog")]
    Dialog,
    #[parse(rename = "get")]
    Get,
    #[parse(rename = "post")]
    Post,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum FmValues {
    #[parse(rename = "get")]
    Get,
    #[parse(rename = "post")]
    Post,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum SValues {
    #[parse(rename = "col")]
    Col,
    #[parse(rename = "colgroup")]
    Colgroup,
    #[parse(rename = "row")]
    Row,
    #[parse(rename = "rowgroup")]
    Rowgroup,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TValues {
    #[parse(rename = "button")]
    Button,
    #[parse(rename = "checkbox")]
    Checkbox,
    #[parse(rename = "color")]
    Color,
    #[parse(rename = "date")]
    Date,
    #[parse(rename = "datetime")]
    Datetime,
    #[parse(rename = "datetime-local")]
    DatetimeLocal,
    #[parse(rename = "email")]
    Email,
    #[parse(rename = "file")]
    File,
    #[parse(rename = "hidden")]
    Hidden,
    #[parse(rename = "image")]
    Image,
    #[parse(rename = "month")]
    Month,
    #[parse(rename = "number")]
    Number,
    #[parse(rename = "password")]
    Password,
    #[parse(rename = "radio")]
    Radio,
    #[parse(rename = "range")]
    Range,
    #[parse(rename = "reset")]
    Reset,
    #[parse(rename = "search")]
    Search,
    #[parse(rename = "submit")]
    Submit,
    #[parse(rename = "tel")]
    Tel,
    #[parse(rename = "text")]
    Text,
    #[parse(rename = "time")]
    Time,
    #[parse(rename = "url")]
    Url,
    #[parse(rename = "week")]
    Week,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum ImValues {
    #[parse(rename = "email")]
    Email,
    #[parse(rename = "full-width-latin")]
    FullWidthLatin,
    #[parse(rename = "kana")]
    Kana,
    #[parse(rename = "kana-name")]
    KanaName,
    #[parse(rename = "katakana")]
    Katakana,
    #[parse(rename = "latin")]
    Latin,
    #[parse(rename = "latin-name")]
    LatinName,
    #[parse(rename = "latin-prose")]
    LatinProse,
    #[parse(rename = "numeric")]
    Numeric,
    #[parse(rename = "tel")]
    Tel,
    #[parse(rename = "url")]
    Url,
    #[parse(rename = "verbatim")]
    Verbatim,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum BtValues {
    #[parse(rename = "button")]
    Button,
    #[parse(rename = "menu")]
    Menu,
    #[parse(rename = "reset")]
    Reset,
    #[parse(rename = "submit")]
    Submit,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum LtValues {
    #[parse(rename = "1")]
    Value1,
    #[parse(rename = "A")]
    A,
    #[parse(rename = "I")]
    I,
    #[parse(rename = "a")]
    A2,
    #[parse(rename = "i")]
    I2,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MtValues {
    #[parse(rename = "context")]
    Context,
    #[parse(rename = "toolbar")]
    Toolbar,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MitValues {
    #[parse(rename = "checkbox")]
    Checkbox,
    #[parse(rename = "command")]
    Command,
    #[parse(rename = "radio")]
    Radio,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum EtValues {
    #[parse(rename = "application/x-www-form-urlencoded")]
    ApplicationxWwwFormUrlencoded,
    #[parse(rename = "multipart/form-data")]
    MultipartformData,
    #[parse(rename = "text/plain")]
    Textplain,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TkValues {
    #[parse(rename = "captions")]
    Captions,
    #[parse(rename = "chapters")]
    Chapters,
    #[parse(rename = "descriptions")]
    Descriptions,
    #[parse(rename = "metadata")]
    Metadata,
    #[parse(rename = "subtitles")]
    Subtitles,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum PlValues {
    #[parse(rename = "auto")]
    Auto,
    #[parse(rename = "metadata")]
    Metadata,
    #[parse(rename = "none")]
    None,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum ShValues {
    #[parse(rename = "circle")]
    Circle,
    #[parse(rename = "default")]
    Default,
    #[parse(rename = "poly")]
    Poly,
    #[parse(rename = "rect")]
    Rect,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum XoValues {
    #[parse(rename = "anonymous")]
    Anonymous,
    #[parse(rename = "use-credentials")]
    UseCredentials,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TargetValues {
    #[parse(rename = "_blank")]
    Blank,
    #[parse(rename = "_parent")]
    Parent,
    #[parse(rename = "_self")]
    ValueSelf,
    #[parse(rename = "_top")]
    Top,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum SbValues {
    #[parse(rename = "allow-forms")]
    AllowForms,
    #[parse(rename = "allow-modals")]
    AllowModals,
    #[parse(rename = "allow-pointer-lock")]
    AllowPointerLock,
    #[parse(rename = "allow-popups")]
    AllowPopups,
    #[parse(rename = "allow-popups-to-escape-sandbox")]
    AllowPopupsToEscapeSandbox,
    #[parse(rename = "allow-same-origin")]
    AllowSameOrigin,
    #[parse(rename = "allow-scripts")]
    AllowScripts,
    #[parse(rename = "allow-top-navigation")]
    AllowTopNavigation,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum TristateValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "mixed")]
    Mixed,
    #[parse(rename = "true")]
    True,
    #[parse(rename = "undefined")]
    Undefined,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum InputautocompleteValues {
    #[parse(rename = "additional-name")]
    AdditionalName,
    #[parse(rename = "address-level1")]
    AddressLevel1,
    #[parse(rename = "address-level2")]
    AddressLevel2,
    #[parse(rename = "address-level3")]
    AddressLevel3,
    #[parse(rename = "address-level4")]
    AddressLevel4,
    #[parse(rename = "address-line1")]
    AddressLine1,
    #[parse(rename = "address-line2")]
    AddressLine2,
    #[parse(rename = "address-line3")]
    AddressLine3,
    #[parse(rename = "bday")]
    Bday,
    #[parse(rename = "bday-day")]
    BdayDay,
    #[parse(rename = "bday-month")]
    BdayMonth,
    #[parse(rename = "bday-year")]
    BdayYear,
    #[parse(rename = "billing")]
    Billing,
    #[parse(rename = "cc-additional-name")]
    CcAdditionalName,
    #[parse(rename = "cc-csc")]
    CcCsc,
    #[parse(rename = "cc-exp")]
    CcExp,
    #[parse(rename = "cc-exp-month")]
    CcExpMonth,
    #[parse(rename = "cc-exp-year")]
    CcExpYear,
    #[parse(rename = "cc-family-name")]
    CcFamilyName,
    #[parse(rename = "cc-given-name")]
    CcGivenName,
    #[parse(rename = "cc-name")]
    CcName,
    #[parse(rename = "cc-number")]
    CcNumber,
    #[parse(rename = "cc-type")]
    CcType,
    #[parse(rename = "country")]
    Country,
    #[parse(rename = "country-name")]
    CountryName,
    #[parse(rename = "current-password")]
    CurrentPassword,
    #[parse(rename = "email")]
    Email,
    #[parse(rename = "family-name")]
    FamilyName,
    #[parse(rename = "fax")]
    Fax,
    #[parse(rename = "given-name")]
    GivenName,
    #[parse(rename = "home")]
    Home,
    #[parse(rename = "honorific-prefix")]
    HonorificPrefix,
    #[parse(rename = "honorific-suffix")]
    HonorificSuffix,
    #[parse(rename = "impp")]
    Impp,
    #[parse(rename = "language")]
    Language,
    #[parse(rename = "mobile")]
    Mobile,
    #[parse(rename = "name")]
    Name,
    #[parse(rename = "new-password")]
    NewPassword,
    #[parse(rename = "nickname")]
    Nickname,
    #[parse(rename = "off")]
    Off,
    #[parse(rename = "on")]
    On,
    #[parse(rename = "organization")]
    Organization,
    #[parse(rename = "organization-title")]
    OrganizationTitle,
    #[parse(rename = "pager")]
    Pager,
    #[parse(rename = "photo")]
    Photo,
    #[parse(rename = "postal-code")]
    PostalCode,
    #[parse(rename = "sex")]
    Sex,
    #[parse(rename = "shipping")]
    Shipping,
    #[parse(rename = "street-address")]
    StreetAddress,
    #[parse(rename = "tel")]
    Tel,
    #[parse(rename = "tel-area-code")]
    TelAreaCode,
    #[parse(rename = "tel-country-code")]
    TelCountryCode,
    #[parse(rename = "tel-extension")]
    TelExtension,
    #[parse(rename = "tel-local")]
    TelLocal,
    #[parse(rename = "tel-local-prefix")]
    TelLocalPrefix,
    #[parse(rename = "tel-local-suffix")]
    TelLocalSuffix,
    #[parse(rename = "tel-national")]
    TelNational,
    #[parse(rename = "transaction-amount")]
    TransactionAmount,
    #[parse(rename = "transaction-currency")]
    TransactionCurrency,
    #[parse(rename = "url")]
    Url,
    #[parse(rename = "username")]
    Username,
    #[parse(rename = "work")]
    Work,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum AutocompleteValues {
    #[parse(rename = "both")]
    Both,
    #[parse(rename = "inline")]
    Inline,
    #[parse(rename = "list")]
    List,
    #[parse(rename = "none")]
    None,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum CurrentValues {
    #[parse(rename = "date")]
    Date,
    #[parse(rename = "false")]
    False,
    #[parse(rename = "location")]
    Location,
    #[parse(rename = "page")]
    Page,
    #[parse(rename = "step")]
    Step,
    #[parse(rename = "time")]
    Time,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum DropeffectValues {
    #[parse(rename = "copy")]
    Copy,
    #[parse(rename = "execute")]
    Execute,
    #[parse(rename = "link")]
    Link,
    #[parse(rename = "move")]
    Move,
    #[parse(rename = "none")]
    None,
    #[parse(rename = "popup")]
    Popup,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum InvalidValues {
    #[parse(rename = "false")]
    False,
    #[parse(rename = "grammar")]
    Grammar,
    #[parse(rename = "spelling")]
    Spelling,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum LiveValues {
    #[parse(rename = "assertive")]
    Assertive,
    #[parse(rename = "off")]
    Off,
    #[parse(rename = "polite")]
    Polite,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum OrientationValues {
    #[parse(rename = "horizontal")]
    Horizontal,
    #[parse(rename = "undefined")]
    Undefined,
    #[parse(rename = "vertical")]
    Vertical,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum RelevantValues {
    #[parse(rename = "additions")]
    Additions,
    #[parse(rename = "additions text")]
    AdditionsText,
    #[parse(rename = "all")]
    All,
    #[parse(rename = "removals")]
    Removals,
    #[parse(rename = "text")]
    Text,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum SortValues {
    #[parse(rename = "ascending")]
    Ascending,
    #[parse(rename = "descending")]
    Descending,
    #[parse(rename = "none")]
    None,
    #[parse(rename = "other")]
    Other,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum RolesValues {
    #[parse(rename = "alert")]
    Alert,
    #[parse(rename = "alertdialog")]
    Alertdialog,
    #[parse(rename = "application")]
    Application,
    #[parse(rename = "article")]
    Article,
    #[parse(rename = "banner")]
    Banner,
    #[parse(rename = "button")]
    Button,
    #[parse(rename = "cell")]
    Cell,
    #[parse(rename = "checkbox")]
    Checkbox,
    #[parse(rename = "columnheader")]
    Columnheader,
    #[parse(rename = "combobox")]
    Combobox,
    #[parse(rename = "complementary")]
    Complementary,
    #[parse(rename = "contentinfo")]
    Contentinfo,
    #[parse(rename = "definition")]
    Definition,
    #[parse(rename = "dialog")]
    Dialog,
    #[parse(rename = "directory")]
    Directory,
    #[parse(rename = "doc-abstract")]
    DocAbstract,
    #[parse(rename = "doc-acknowledgments")]
    DocAcknowledgments,
    #[parse(rename = "doc-afterword")]
    DocAfterword,
    #[parse(rename = "doc-appendix")]
    DocAppendix,
    #[parse(rename = "doc-backlink")]
    DocBacklink,
    #[parse(rename = "doc-biblioentry")]
    DocBiblioentry,
    #[parse(rename = "doc-bibliography")]
    DocBibliography,
    #[parse(rename = "doc-biblioref")]
    DocBiblioref,
    #[parse(rename = "doc-chapter")]
    DocChapter,
    #[parse(rename = "doc-colophon")]
    DocColophon,
    #[parse(rename = "doc-conclusion")]
    DocConclusion,
    #[parse(rename = "doc-cover")]
    DocCover,
    #[parse(rename = "doc-credit")]
    DocCredit,
    #[parse(rename = "doc-credits")]
    DocCredits,
    #[parse(rename = "doc-dedication")]
    DocDedication,
    #[parse(rename = "doc-endnote")]
    DocEndnote,
    #[parse(rename = "doc-endnotes")]
    DocEndnotes,
    #[parse(rename = "doc-epigraph")]
    DocEpigraph,
    #[parse(rename = "doc-epilogue")]
    DocEpilogue,
    #[parse(rename = "doc-errata")]
    DocErrata,
    #[parse(rename = "doc-example")]
    DocExample,
    #[parse(rename = "doc-footnote")]
    DocFootnote,
    #[parse(rename = "doc-foreword")]
    DocForeword,
    #[parse(rename = "doc-glossary")]
    DocGlossary,
    #[parse(rename = "doc-glossref")]
    DocGlossref,
    #[parse(rename = "doc-index")]
    DocIndex,
    #[parse(rename = "doc-introduction")]
    DocIntroduction,
    #[parse(rename = "doc-noteref")]
    DocNoteref,
    #[parse(rename = "doc-notice")]
    DocNotice,
    #[parse(rename = "doc-pagebreak")]
    DocPagebreak,
    #[parse(rename = "doc-pagelist")]
    DocPagelist,
    #[parse(rename = "doc-part")]
    DocPart,
    #[parse(rename = "doc-preface")]
    DocPreface,
    #[parse(rename = "doc-prologue")]
    DocPrologue,
    #[parse(rename = "doc-pullquote")]
    DocPullquote,
    #[parse(rename = "doc-qna")]
    DocQna,
    #[parse(rename = "doc-subtitle")]
    DocSubtitle,
    #[parse(rename = "doc-tip")]
    DocTip,
    #[parse(rename = "doc-toc")]
    DocToc,
    #[parse(rename = "document")]
    Document,
    #[parse(rename = "feed")]
    Feed,
    #[parse(rename = "figure")]
    Figure,
    #[parse(rename = "form")]
    Form,
    #[parse(rename = "grid")]
    Grid,
    #[parse(rename = "gridcell")]
    Gridcell,
    #[parse(rename = "group")]
    Group,
    #[parse(rename = "heading")]
    Heading,
    #[parse(rename = "img")]
    Img,
    #[parse(rename = "link")]
    Link,
    #[parse(rename = "list")]
    List,
    #[parse(rename = "listbox")]
    Listbox,
    #[parse(rename = "listitem")]
    Listitem,
    #[parse(rename = "log")]
    Log,
    #[parse(rename = "main")]
    Main,
    #[parse(rename = "marquee")]
    Marquee,
    #[parse(rename = "math")]
    Math,
    #[parse(rename = "menu")]
    Menu,
    #[parse(rename = "menubar")]
    Menubar,
    #[parse(rename = "menuitem")]
    Menuitem,
    #[parse(rename = "menuitemcheckbox")]
    Menuitemcheckbox,
    #[parse(rename = "menuitemradio")]
    Menuitemradio,
    #[parse(rename = "navigation")]
    Navigation,
    #[parse(rename = "none")]
    None,
    #[parse(rename = "note")]
    Note,
    #[parse(rename = "option")]
    Option,
    #[parse(rename = "presentation")]
    Presentation,
    #[parse(rename = "progressbar")]
    Progressbar,
    #[parse(rename = "radio")]
    Radio,
    #[parse(rename = "radiogroup")]
    Radiogroup,
    #[parse(rename = "region")]
    Region,
    #[parse(rename = "row")]
    Row,
    #[parse(rename = "rowgroup")]
    Rowgroup,
    #[parse(rename = "rowheader")]
    Rowheader,
    #[parse(rename = "scrollbar")]
    Scrollbar,
    #[parse(rename = "search")]
    Search,
    #[parse(rename = "searchbox")]
    Searchbox,
    #[parse(rename = "separator")]
    Separator,
    #[parse(rename = "slider")]
    Slider,
    #[parse(rename = "spinbutton")]
    Spinbutton,
    #[parse(rename = "status")]
    Status,
    #[parse(rename = "switch")]
    Switch,
    #[parse(rename = "tab")]
    Tab,
    #[parse(rename = "table")]
    Table,
    #[parse(rename = "tablist")]
    Tablist,
    #[parse(rename = "tabpanel")]
    Tabpanel,
    #[parse(rename = "term")]
    Term,
    #[parse(rename = "text")]
    Text,
    #[parse(rename = "textbox")]
    Textbox,
    #[parse(rename = "timer")]
    Timer,
    #[parse(rename = "toolbar")]
    Toolbar,
    #[parse(rename = "tooltip")]
    Tooltip,
    #[parse(rename = "tree")]
    Tree,
    #[parse(rename = "treegrid")]
    Treegrid,
    #[parse(rename = "treeitem")]
    Treeitem,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum MetanamesValues {
    #[parse(rename = "application-name")]
    ApplicationName,
    #[parse(rename = "author")]
    Author,
    #[parse(rename = "description")]
    Description,
    #[parse(rename = "format-detection")]
    FormatDetection,
    #[parse(rename = "generator")]
    Generator,
    #[parse(rename = "keywords")]
    Keywords,
    #[parse(rename = "publisher")]
    Publisher,
    #[parse(rename = "referrer")]
    Referrer,
    #[parse(rename = "robots")]
    Robots,
    #[parse(rename = "theme-color")]
    ThemeColor,
    #[parse(rename = "viewport")]
    Viewport,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum HaspopupValues {
    #[parse(rename = "dialog")]
    Dialog,
    #[parse(rename = "false")]
    False,
    #[parse(rename = "grid")]
    Grid,
    #[parse(rename = "listbox")]
    Listbox,
    #[parse(rename = "menu")]
    Menu,
    #[parse(rename = "tree")]
    Tree,
    #[parse(rename = "true")]
    True,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum DecodingValues {
    #[parse(rename = "async")]
    Async,
    #[parse(rename = "auto")]
    Auto,
    #[parse(rename = "sync")]
    Sync,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum LoadingValues {
    #[parse(rename = "eager")]
    Eager,
    #[parse(rename = "lazy")]
    Lazy,
}
#[derive(Debug, Clone, Copy, Parse)]
pub enum ReferrerpolicyValues {
    #[parse(rename = "no-referrer")]
    NoReferrer,
    #[parse(rename = "no-referrer-when-downgrade")]
    NoReferrerWhenDowngrade,
    #[parse(rename = "origin")]
    Origin,
    #[parse(rename = "origin-when-cross-origin")]
    OriginWhenCrossOrigin,
    #[parse(rename = "same-origin")]
    SameOrigin,
    #[parse(rename = "strict-origin")]
    StrictOrigin,
    #[parse(rename = "strict-origin-when-cross-origin")]
    StrictOriginWhenCrossOrigin,
    #[parse(rename = "unsafe-url")]
    UnsafeUrl,
}
mod a;
pub use a::*;


mod abbr;
pub use abbr::*;


mod address;
pub use address::*;


mod area;
pub use area::*;


mod article;
pub use article::*;


mod aside;
pub use aside::*;


mod audio;
pub use audio::*;


mod b;
pub use b::*;


mod base;
pub use base::*;


mod bdi;
pub use bdi::*;


mod bdo;
pub use bdo::*;


mod blockquote;
pub use blockquote::*;


mod body;
pub use body::*;


mod br;
pub use br::*;


mod button;
pub use button::*;


mod canvas;
pub use canvas::*;


mod caption;
pub use caption::*;


mod cite;
pub use cite::*;


mod code;
pub use code::*;


mod col;
pub use col::*;


mod colgroup;
pub use colgroup::*;


mod data;
pub use data::*;


mod datalist;
pub use datalist::*;


mod dd;
pub use dd::*;


mod del;
pub use del::*;


mod details;
pub use details::*;


mod dfn;
pub use dfn::*;


mod dialog;
pub use dialog::*;


mod div;
pub use div::*;


mod dl;
pub use dl::*;


mod dt;
pub use dt::*;


mod em;
pub use em::*;


mod embed;
pub use embed::*;


mod fieldset;
pub use fieldset::*;


mod figcaption;
pub use figcaption::*;


mod figure;
pub use figure::*;


mod footer;
pub use footer::*;


mod form;
pub use form::*;


mod h1;
pub use h1::*;


mod h2;
pub use h2::*;


mod h3;
pub use h3::*;


mod h4;
pub use h4::*;


mod h5;
pub use h5::*;


mod h6;
pub use h6::*;


mod head;
pub use head::*;


mod header;
pub use header::*;


mod hgroup;
pub use hgroup::*;


mod hr;
pub use hr::*;


mod html;
pub use html::*;


mod i;
pub use i::*;


mod iframe;
pub use iframe::*;


mod img;
pub use img::*;


mod input;
pub use input::*;


mod ins;
pub use ins::*;


mod kbd;
pub use kbd::*;


mod label;
pub use label::*;


mod legend;
pub use legend::*;


mod li;
pub use li::*;


mod link;
pub use link::*;


mod main;
pub use main::*;


mod map;
pub use map::*;


mod mark;
pub use mark::*;


mod menu;
pub use menu::*;


mod meta;
pub use meta::*;


mod meter;
pub use meter::*;


mod nav;
pub use nav::*;


mod noscript;
pub use noscript::*;


mod object;
pub use object::*;


mod ol;
pub use ol::*;


mod optgroup;
pub use optgroup::*;


mod option;
pub use option::*;


mod output;
pub use output::*;


mod p;
pub use p::*;


mod param;
pub use param::*;


mod picture;
pub use picture::*;


mod pre;
pub use pre::*;


mod progress;
pub use progress::*;


mod q;
pub use q::*;


mod rb;
pub use rb::*;


mod rp;
pub use rp::*;


mod rt;
pub use rt::*;


mod ruby;
pub use ruby::*;


mod s;
pub use s::*;


mod samp;
pub use samp::*;


mod script;
pub use script::*;


mod section;
pub use section::*;


mod select;
pub use select::*;


mod slot;
pub use slot::*;


mod small;
pub use small::*;


mod source;
pub use source::*;


mod span;
pub use span::*;


mod strong;
pub use strong::*;


mod style;
pub use style::*;


mod sub;
pub use sub::*;


mod summary;
pub use summary::*;


mod sup;
pub use sup::*;


mod table;
pub use table::*;


mod tbody;
pub use tbody::*;


mod td;
pub use td::*;


mod template;
pub use template::*;


mod textarea;
pub use textarea::*;


mod tfoot;
pub use tfoot::*;


mod th;
pub use th::*;


mod thead;
pub use thead::*;


mod time;
pub use time::*;


mod title;
pub use title::*;


mod tr;
pub use tr::*;


mod track;
pub use track::*;


mod u;
pub use u::*;


mod ul;
pub use ul::*;


mod var;
pub use var::*;


mod video;
pub use video::*;


mod wbr;
pub use wbr::*;


#[derive(Debug, Clone)]
pub enum Element {
    A(A),
    Abbr(Abbr),
    Address(Address),
    Area(Area),
    Article(Article),
    Aside(Aside),
    Audio(Audio),
    B(B),
    Base(Base),
    Bdi(Bdi),
    Bdo(Bdo),
    Blockquote(Blockquote),
    Body(Body),
    Br(Br),
    Button(Button),
    Canvas(Canvas),
    Caption(Caption),
    Cite(Cite),
    Code(Code),
    Col(Col),
    Colgroup(Colgroup),
    Data(Data),
    Datalist(Datalist),
    Dd(Dd),
    Del(Del),
    Details(Details),
    Dfn(Dfn),
    Dialog(Dialog),
    Div(Div),
    Dl(Dl),
    Dt(Dt),
    Em(Em),
    Embed(Embed),
    Fieldset(Fieldset),
    Figcaption(Figcaption),
    Figure(Figure),
    Footer(Footer),
    Form(Form),
    H1(H1),
    H2(H2),
    H3(H3),
    H4(H4),
    H5(H5),
    H6(H6),
    Head(Head),
    Header(Header),
    Hgroup(Hgroup),
    Hr(Hr),
    Html(Html),
    I(I),
    Iframe(Iframe),
    Img(Img),
    Input(Input),
    Ins(Ins),
    Kbd(Kbd),
    Label(Label),
    Legend(Legend),
    Li(Li),
    Link(Link),
    Main(Main),
    Map(Map),
    Mark(Mark),
    Menu(Menu),
    Meta(Meta),
    Meter(Meter),
    Nav(Nav),
    Noscript(Noscript),
    Object(Object),
    Ol(Ol),
    Optgroup(Optgroup),
    Option(Option),
    Output(Output),
    P(P),
    Param(Param),
    Picture(Picture),
    Pre(Pre),
    Progress(Progress),
    Q(Q),
    Rb(Rb),
    Rp(Rp),
    Rt(Rt),
    Ruby(Ruby),
    S(S),
    Samp(Samp),
    Script(Script),
    Section(Section),
    Select(Select),
    Slot(Slot),
    Small(Small),
    Source(Source),
    Span(Span),
    Strong(Strong),
    Style(Style),
    Sub(Sub),
    Summary(Summary),
    Sup(Sup),
    Table(Table),
    Tbody(Tbody),
    Td(Td),
    Template(Template),
    Textarea(Textarea),
    Tfoot(Tfoot),
    Th(Th),
    Thead(Thead),
    Time(Time),
    Title(Title),
    Tr(Tr),
    Track(Track),
    U(U),
    Ul(Ul),
    Var(Var),
    Video(Video),
    Wbr(Wbr),
}
impl kalosm_sample::Parse for Element {
    fn new_parser() -> impl kalosm_sample::SendCreateParserState<Output = Self> {
        use kalosm_sample::*;
        A::new_parser().map_output(Self::A)
        .or(
        Abbr::new_parser().map_output(Self::Abbr)
        )
        .or(
        Address::new_parser().map_output(Self::Address)
        )
        .or(
        Area::new_parser().map_output(Self::Area)
        )
        .or(
        Article::new_parser().map_output(Self::Article)
        )
        .or(
        Aside::new_parser().map_output(Self::Aside)
        )
        .or(
        Audio::new_parser().map_output(Self::Audio)
        )
        .or(
        B::new_parser().map_output(Self::B)
        )
        .or(
        Base::new_parser().map_output(Self::Base)
        )
        .or(
        Bdi::new_parser().map_output(Self::Bdi)
        )
        .or(
        Bdo::new_parser().map_output(Self::Bdo)
        )
        .or(
        Blockquote::new_parser().map_output(Self::Blockquote)
        )
        .or(
        Body::new_parser().map_output(Self::Body)
        )
        .or(
        Br::new_parser().map_output(Self::Br)
        )
        .or(
        Button::new_parser().map_output(Self::Button)
        )
        .or(
        Canvas::new_parser().map_output(Self::Canvas)
        )
        .or(
        Caption::new_parser().map_output(Self::Caption)
        )
        .or(
        Cite::new_parser().map_output(Self::Cite)
        )
        .or(
        Code::new_parser().map_output(Self::Code)
        )
        .or(
        Col::new_parser().map_output(Self::Col)
        )
        .or(
        Colgroup::new_parser().map_output(Self::Colgroup)
        )
        .or(
        Data::new_parser().map_output(Self::Data)
        )
        .or(
        Datalist::new_parser().map_output(Self::Datalist)
        )
        .or(
        Dd::new_parser().map_output(Self::Dd)
        )
        .or(
        Del::new_parser().map_output(Self::Del)
        )
        .or(
        Details::new_parser().map_output(Self::Details)
        )
        .or(
        Dfn::new_parser().map_output(Self::Dfn)
        )
        .or(
        Dialog::new_parser().map_output(Self::Dialog)
        )
        .or(
        Div::new_parser().map_output(Self::Div)
        )
        .or(
        Dl::new_parser().map_output(Self::Dl)
        )
        .or(
        Dt::new_parser().map_output(Self::Dt)
        )
        .or(
        Em::new_parser().map_output(Self::Em)
        )
        .or(
        Embed::new_parser().map_output(Self::Embed)
        )
        .or(
        Fieldset::new_parser().map_output(Self::Fieldset)
        )
        .or(
        Figcaption::new_parser().map_output(Self::Figcaption)
        )
        .or(
        Figure::new_parser().map_output(Self::Figure)
        )
        .or(
        Footer::new_parser().map_output(Self::Footer)
        )
        .or(
        Form::new_parser().map_output(Self::Form)
        )
        .or(
        H1::new_parser().map_output(Self::H1)
        )
        .or(
        H2::new_parser().map_output(Self::H2)
        )
        .or(
        H3::new_parser().map_output(Self::H3)
        )
        .or(
        H4::new_parser().map_output(Self::H4)
        )
        .or(
        H5::new_parser().map_output(Self::H5)
        )
        .or(
        H6::new_parser().map_output(Self::H6)
        )
        .or(
        Head::new_parser().map_output(Self::Head)
        )
        .or(
        Header::new_parser().map_output(Self::Header)
        )
        .or(
        Hgroup::new_parser().map_output(Self::Hgroup)
        )
        .or(
        Hr::new_parser().map_output(Self::Hr)
        )
        .or(
        Html::new_parser().map_output(Self::Html)
        )
        .or(
        I::new_parser().map_output(Self::I)
        )
        .or(
        Iframe::new_parser().map_output(Self::Iframe)
        )
        .or(
        Img::new_parser().map_output(Self::Img)
        )
        .or(
        Input::new_parser().map_output(Self::Input)
        )
        .or(
        Ins::new_parser().map_output(Self::Ins)
        )
        .or(
        Kbd::new_parser().map_output(Self::Kbd)
        )
        .or(
        Label::new_parser().map_output(Self::Label)
        )
        .or(
        Legend::new_parser().map_output(Self::Legend)
        )
        .or(
        Li::new_parser().map_output(Self::Li)
        )
        .or(
        Link::new_parser().map_output(Self::Link)
        )
        .or(
        Main::new_parser().map_output(Self::Main)
        )
        .or(
        Map::new_parser().map_output(Self::Map)
        )
        .or(
        Mark::new_parser().map_output(Self::Mark)
        )
        .or(
        Menu::new_parser().map_output(Self::Menu)
        )
        .or(
        Meta::new_parser().map_output(Self::Meta)
        )
        .or(
        Meter::new_parser().map_output(Self::Meter)
        )
        .or(
        Nav::new_parser().map_output(Self::Nav)
        )
        .or(
        Noscript::new_parser().map_output(Self::Noscript)
        )
        .or(
        Object::new_parser().map_output(Self::Object)
        )
        .or(
        Ol::new_parser().map_output(Self::Ol)
        )
        .or(
        Optgroup::new_parser().map_output(Self::Optgroup)
        )
        .or(
        Option::new_parser().map_output(Self::Option)
        )
        .or(
        Output::new_parser().map_output(Self::Output)
        )
        .or(
        P::new_parser().map_output(Self::P)
        )
        .or(
        Param::new_parser().map_output(Self::Param)
        )
        .or(
        Picture::new_parser().map_output(Self::Picture)
        )
        .or(
        Pre::new_parser().map_output(Self::Pre)
        )
        .or(
        Progress::new_parser().map_output(Self::Progress)
        )
        .or(
        Q::new_parser().map_output(Self::Q)
        )
        .or(
        Rb::new_parser().map_output(Self::Rb)
        )
        .or(
        Rp::new_parser().map_output(Self::Rp)
        )
        .or(
        Rt::new_parser().map_output(Self::Rt)
        )
        .or(
        Ruby::new_parser().map_output(Self::Ruby)
        )
        .or(
        S::new_parser().map_output(Self::S)
        )
        .or(
        Samp::new_parser().map_output(Self::Samp)
        )
        .or(
        Script::new_parser().map_output(Self::Script)
        )
        .or(
        Section::new_parser().map_output(Self::Section)
        )
        .or(
        Select::new_parser().map_output(Self::Select)
        )
        .or(
        Slot::new_parser().map_output(Self::Slot)
        )
        .or(
        Small::new_parser().map_output(Self::Small)
        )
        .or(
        Source::new_parser().map_output(Self::Source)
        )
        .or(
        Span::new_parser().map_output(Self::Span)
        )
        .or(
        Strong::new_parser().map_output(Self::Strong)
        )
        .or(
        Style::new_parser().map_output(Self::Style)
        )
        .or(
        Sub::new_parser().map_output(Self::Sub)
        )
        .or(
        Summary::new_parser().map_output(Self::Summary)
        )
        .or(
        Sup::new_parser().map_output(Self::Sup)
        )
        .or(
        Table::new_parser().map_output(Self::Table)
        )
        .or(
        Tbody::new_parser().map_output(Self::Tbody)
        )
        .or(
        Td::new_parser().map_output(Self::Td)
        )
        .or(
        Template::new_parser().map_output(Self::Template)
        )
        .or(
        Textarea::new_parser().map_output(Self::Textarea)
        )
        .or(
        Tfoot::new_parser().map_output(Self::Tfoot)
        )
        .or(
        Th::new_parser().map_output(Self::Th)
        )
        .or(
        Thead::new_parser().map_output(Self::Thead)
        )
        .or(
        Time::new_parser().map_output(Self::Time)
        )
        .or(
        Title::new_parser().map_output(Self::Title)
        )
        .or(
        Tr::new_parser().map_output(Self::Tr)
        )
        .or(
        Track::new_parser().map_output(Self::Track)
        )
        .or(
        U::new_parser().map_output(Self::U)
        )
        .or(
        Ul::new_parser().map_output(Self::Ul)
        )
        .or(
        Var::new_parser().map_output(Self::Var)
        )
        .or(
        Video::new_parser().map_output(Self::Video)
        )
        .or(
        Wbr::new_parser().map_output(Self::Wbr)
        )
    }
}
