#[doc = "Register `PROC1_NMI_MASK` reader"]
pub struct R(crate::R<PROC1_NMI_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PROC1_NMI_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PROC1_NMI_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PROC1_NMI_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PROC1_NMI_MASK` writer"]
pub struct W(crate::W<PROC1_NMI_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PROC1_NMI_MASK_SPEC>;
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
impl From<crate::W<PROC1_NMI_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PROC1_NMI_MASK_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Processor core 1 NMI source mask  
 Set a bit high to enable NMI from that IRQ  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [proc1_nmi_mask](index.html) module"]
pub struct PROC1_NMI_MASK_SPEC;
impl crate::RegisterSpec for PROC1_NMI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [proc1_nmi_mask::R](R) reader structure"]
impl crate::Readable for PROC1_NMI_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [proc1_nmi_mask::W](W) writer structure"]
impl crate::Writable for PROC1_NMI_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PROC1_NMI_MASK to value 0"]
impl crate::Resettable for PROC1_NMI_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
