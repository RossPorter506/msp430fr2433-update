#[doc = "Register `P2IFG` reader"]
pub struct R(crate::R<P2IFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P2IFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P2IFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P2IFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P2IFG` writer"]
pub struct W(crate::W<P2IFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P2IFG_SPEC>;
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
impl From<crate::W<P2IFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P2IFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P2IFG` reader - P2IFG"]
pub type P2IFG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P2IFG` writer - P2IFG"]
pub type P2IFG_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, P2IFG_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - P2IFG"]
    #[inline(always)]
    pub fn p2ifg(&self) -> P2IFG_R {
        P2IFG_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P2IFG"]
    #[inline(always)]
    pub fn p2ifg(&mut self) -> P2IFG_W<0> {
        P2IFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Port 2 Interrupt Flag register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p2ifg](index.html) module"]
pub struct P2IFG_SPEC;
impl crate::RegisterSpec for P2IFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p2ifg::R](R) reader structure"]
impl crate::Readable for P2IFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p2ifg::W](W) writer structure"]
impl crate::Writable for P2IFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P2IFG to value 0"]
impl crate::Resettable for P2IFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
