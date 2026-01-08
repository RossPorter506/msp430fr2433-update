#[doc = "Register `PMMCTL2` reader"]
pub struct R(crate::R<PMMCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL2` writer"]
pub struct W(crate::W<PMMCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PMMCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTREFEN` reader - Internal Reference Enable"]
pub type INTREFEN_R = crate::BitReader<bool>;
#[doc = "Field `INTREFEN` writer - Internal Reference Enable"]
pub type INTREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `EXTREFEN` reader - External Reference output Enable"]
pub type EXTREFEN_R = crate::BitReader<bool>;
#[doc = "Field `EXTREFEN` writer - External Reference output Enable"]
pub type EXTREFEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `TSENSOREN` reader - Temperature Sensor Enable"]
pub type TSENSOREN_R = crate::BitReader<bool>;
#[doc = "Field `TSENSOREN` writer - Temperature Sensor Enable"]
pub type TSENSOREN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `REFVSEL` reader - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
pub type REFVSEL_R = crate::FieldReader<u8, REFVSEL_A>;
#[doc = "Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REFVSEL_A {
    #[doc = "0: 00b = 1.5V"]
    REFVSEL_0 = 0,
    #[doc = "1: 01b = 2.0V"]
    REFVSEL_1 = 1,
    #[doc = "2: 10b = 2.5V"]
    REFVSEL_2 = 2,
    #[doc = "3: 11b = Reserved"]
    REFVSEL_3 = 3,
}
impl From<REFVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: REFVSEL_A) -> Self {
        variant as _
    }
}
impl REFVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REFVSEL_A {
        match self.bits {
            0 => REFVSEL_A::REFVSEL_0,
            1 => REFVSEL_A::REFVSEL_1,
            2 => REFVSEL_A::REFVSEL_2,
            3 => REFVSEL_A::REFVSEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `REFVSEL_0`"]
    #[inline(always)]
    pub fn is_refvsel_0(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_0
    }
    #[doc = "Checks if the value of the field is `REFVSEL_1`"]
    #[inline(always)]
    pub fn is_refvsel_1(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_1
    }
    #[doc = "Checks if the value of the field is `REFVSEL_2`"]
    #[inline(always)]
    pub fn is_refvsel_2(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_2
    }
    #[doc = "Checks if the value of the field is `REFVSEL_3`"]
    #[inline(always)]
    pub fn is_refvsel_3(&self) -> bool {
        *self == REFVSEL_A::REFVSEL_3
    }
}
#[doc = "Field `REFVSEL` writer - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
pub type REFVSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, PMMCTL2_SPEC, u8, REFVSEL_A, 2, O>;
impl<'a, const O: u8> REFVSEL_W<'a, O> {
    #[doc = "00b = 1.5V"]
    #[inline(always)]
    pub fn refvsel_0(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_0)
    }
    #[doc = "01b = 2.0V"]
    #[inline(always)]
    pub fn refvsel_1(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_1)
    }
    #[doc = "10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel_2(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_2)
    }
    #[doc = "11b = Reserved"]
    #[inline(always)]
    pub fn refvsel_3(self) -> &'a mut W {
        self.variant(REFVSEL_A::REFVSEL_3)
    }
}
#[doc = "Field `REFGEN` reader - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
pub type REFGEN_R = crate::BitReader<bool>;
#[doc = "Field `REFGEN` writer - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
pub type REFGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `REFBGEN` reader - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
pub type REFBGEN_R = crate::BitReader<bool>;
#[doc = "Field `REFBGEN` writer - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
pub type REFBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `REFGENACT` reader - REF Reference generator active"]
pub type REFGENACT_R = crate::BitReader<bool>;
#[doc = "Field `REFGENACT` writer - REF Reference generator active"]
pub type REFGENACT_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `REFBGACT` reader - REF Reference bandgap active"]
pub type REFBGACT_R = crate::BitReader<bool>;
#[doc = "Field `REFBGACT` writer - REF Reference bandgap active"]
pub type REFBGACT_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `BGMODE` reader - REF Bandgap mode"]
pub type BGMODE_R = crate::BitReader<bool>;
#[doc = "Field `BGMODE` writer - REF Bandgap mode"]
pub type BGMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `REFGENRDY` reader - REF Reference generator ready"]
pub type REFGENRDY_R = crate::BitReader<bool>;
#[doc = "Field `REFGENRDY` writer - REF Reference generator ready"]
pub type REFGENRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
#[doc = "Field `REFBGRDY` reader - REF Reference bandgap ready"]
pub type REFBGRDY_R = crate::BitReader<bool>;
#[doc = "Field `REFBGRDY` writer - REF Reference bandgap ready"]
pub type REFBGRDY_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&self) -> INTREFEN_R {
        INTREFEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&self) -> EXTREFEN_R {
        EXTREFEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&self) -> TSENSOREN_R {
        TSENSOREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel(&self) -> REFVSEL_R {
        REFVSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refgen(&self) -> REFGEN_R {
        REFGEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refbgen(&self) -> REFBGEN_R {
        REFBGEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&self) -> REFGENACT_R {
        REFGENACT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&self) -> REFBGACT_R {
        REFBGACT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&self) -> BGMODE_R {
        BGMODE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&self) -> REFGENRDY_R {
        REFGENRDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&self) -> REFBGRDY_R {
        REFBGRDY_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Reference Enable"]
    #[inline(always)]
    pub fn intrefen(&mut self) -> INTREFEN_W<0> {
        INTREFEN_W::new(self)
    }
    #[doc = "Bit 1 - External Reference output Enable"]
    #[inline(always)]
    pub fn extrefen(&mut self) -> EXTREFEN_W<1> {
        EXTREFEN_W::new(self)
    }
    #[doc = "Bit 3 - Temperature Sensor Enable"]
    #[inline(always)]
    pub fn tsensoren(&mut self) -> TSENSOREN_W<3> {
        TSENSOREN_W::new(self)
    }
    #[doc = "Bits 4:5 - Internal reference voltage level select. 00b = 1.5V, 01b = 2.0V, 10b = 2.5V"]
    #[inline(always)]
    pub fn refvsel(&mut self) -> REFVSEL_W<4> {
        REFVSEL_W::new(self)
    }
    #[doc = "Bit 6 - Reference generator trigger. If written with a 1, the generation of the variable reference voltage is started. When the reference voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refgen(&mut self) -> REFGEN_W<6> {
        REFGEN_W::new(self)
    }
    #[doc = "Bit 7 - Bandgap and bandgap buffer trigger. If written with a 1, the generation of the buffered bandgap voltage is started. When the bandgap buffer voltage request is set, this bit is cleared by hardware or writing 0."]
    #[inline(always)]
    pub fn refbgen(&mut self) -> REFBGEN_W<7> {
        REFBGEN_W::new(self)
    }
    #[doc = "Bit 8 - REF Reference generator active"]
    #[inline(always)]
    pub fn refgenact(&mut self) -> REFGENACT_W<8> {
        REFGENACT_W::new(self)
    }
    #[doc = "Bit 9 - REF Reference bandgap active"]
    #[inline(always)]
    pub fn refbgact(&mut self) -> REFBGACT_W<9> {
        REFBGACT_W::new(self)
    }
    #[doc = "Bit 11 - REF Bandgap mode"]
    #[inline(always)]
    pub fn bgmode(&mut self) -> BGMODE_W<11> {
        BGMODE_W::new(self)
    }
    #[doc = "Bit 12 - REF Reference generator ready"]
    #[inline(always)]
    pub fn refgenrdy(&mut self) -> REFGENRDY_W<12> {
        REFGENRDY_W::new(self)
    }
    #[doc = "Bit 13 - REF Reference bandgap ready"]
    #[inline(always)]
    pub fn refbgrdy(&mut self) -> REFBGRDY_W<13> {
        REFBGRDY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl2](index.html) module"]
pub struct PMMCTL2_SPEC;
impl crate::RegisterSpec for PMMCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl2::R](R) reader structure"]
impl crate::Readable for PMMCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl2::W](W) writer structure"]
impl crate::Writable for PMMCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL2 to value 0"]
impl crate::Resettable for PMMCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
