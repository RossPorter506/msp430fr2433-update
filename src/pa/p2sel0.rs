#[doc = "Register `P2SEL0` reader"]
pub struct R(crate::R<P2SEL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2SEL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2SEL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2SEL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2SEL0` writer"]
pub struct W(crate::W<P2SEL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2SEL0_SPEC>;
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
impl From<crate::W<P2SEL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2SEL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2SEL0_0` reader - P2SEL0_0"]
pub type P2SEL0_0_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_0` writer - P2SEL0_0"]
pub type P2SEL0_0_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
#[doc = "Field `P2SEL0_1` reader - P2SEL0_1"]
pub type P2SEL0_1_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_1` writer - P2SEL0_1"]
pub type P2SEL0_1_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
#[doc = "Field `P2SEL0_2` reader - P2SEL0_2"]
pub type P2SEL0_2_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_2` writer - P2SEL0_2"]
pub type P2SEL0_2_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
#[doc = "Field `P2SEL0_3` reader - P2SEL0_3"]
pub type P2SEL0_3_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_3` writer - P2SEL0_3"]
pub type P2SEL0_3_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
#[doc = "Field `P2SEL0_4` reader - P2SEL0_4"]
pub type P2SEL0_4_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_4` writer - P2SEL0_4"]
pub type P2SEL0_4_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
#[doc = "Field `P2SEL0_5` reader - P2SEL0_5"]
pub type P2SEL0_5_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_5` writer - P2SEL0_5"]
pub type P2SEL0_5_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
#[doc = "Field `P2SEL0_6` reader - P2SEL0_6"]
pub type P2SEL0_6_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_6` writer - P2SEL0_6"]
pub type P2SEL0_6_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
#[doc = "Field `P2SEL0_7` reader - P2SEL0_7"]
pub type P2SEL0_7_R = crate::BitReader<bool>;
#[doc = "Field `P2SEL0_7` writer - P2SEL0_7"]
pub type P2SEL0_7_W<'a, const O: u8> = crate::BitWriter<'a, u8, P2SEL0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - P2SEL0_0"]
    #[inline(always)]
    pub fn p2sel0_0(&self) -> P2SEL0_0_R {
        P2SEL0_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P2SEL0_1"]
    #[inline(always)]
    pub fn p2sel0_1(&self) -> P2SEL0_1_R {
        P2SEL0_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P2SEL0_2"]
    #[inline(always)]
    pub fn p2sel0_2(&self) -> P2SEL0_2_R {
        P2SEL0_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P2SEL0_3"]
    #[inline(always)]
    pub fn p2sel0_3(&self) -> P2SEL0_3_R {
        P2SEL0_3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P2SEL0_4"]
    #[inline(always)]
    pub fn p2sel0_4(&self) -> P2SEL0_4_R {
        P2SEL0_4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P2SEL0_5"]
    #[inline(always)]
    pub fn p2sel0_5(&self) -> P2SEL0_5_R {
        P2SEL0_5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P2SEL0_6"]
    #[inline(always)]
    pub fn p2sel0_6(&self) -> P2SEL0_6_R {
        P2SEL0_6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P2SEL0_7"]
    #[inline(always)]
    pub fn p2sel0_7(&self) -> P2SEL0_7_R {
        P2SEL0_7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P2SEL0_0"]
    #[inline(always)]
    pub fn p2sel0_0(&mut self) -> P2SEL0_0_W<0> {
        P2SEL0_0_W::new(self)
    }
    #[doc = "Bit 1 - P2SEL0_1"]
    #[inline(always)]
    pub fn p2sel0_1(&mut self) -> P2SEL0_1_W<1> {
        P2SEL0_1_W::new(self)
    }
    #[doc = "Bit 2 - P2SEL0_2"]
    #[inline(always)]
    pub fn p2sel0_2(&mut self) -> P2SEL0_2_W<2> {
        P2SEL0_2_W::new(self)
    }
    #[doc = "Bit 3 - P2SEL0_3"]
    #[inline(always)]
    pub fn p2sel0_3(&mut self) -> P2SEL0_3_W<3> {
        P2SEL0_3_W::new(self)
    }
    #[doc = "Bit 4 - P2SEL0_4"]
    #[inline(always)]
    pub fn p2sel0_4(&mut self) -> P2SEL0_4_W<4> {
        P2SEL0_4_W::new(self)
    }
    #[doc = "Bit 5 - P2SEL0_5"]
    #[inline(always)]
    pub fn p2sel0_5(&mut self) -> P2SEL0_5_W<5> {
        P2SEL0_5_W::new(self)
    }
    #[doc = "Bit 6 - P2SEL0_6"]
    #[inline(always)]
    pub fn p2sel0_6(&mut self) -> P2SEL0_6_W<6> {
        P2SEL0_6_W::new(self)
    }
    #[doc = "Bit 7 - P2SEL0_7"]
    #[inline(always)]
    pub fn p2sel0_7(&mut self) -> P2SEL0_7_W<7> {
        P2SEL0_7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port 2 Selection 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2sel0](index.html) module"]
pub struct P2SEL0_SPEC;
impl crate::RegisterSpec for P2SEL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2sel0::R](R) reader structure"]
impl crate::Readable for P2SEL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2sel0::W](W) writer structure"]
impl crate::Writable for P2SEL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2SEL0 to value 0"]
impl crate::Resettable for P2SEL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
