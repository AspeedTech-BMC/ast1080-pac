#[doc = "Register `HCIPIO018` reader"]
pub type R = crate::R<Hcipio018Spec>;
#[doc = "Register `HCIPIO018` writer"]
pub type W = crate::W<Hcipio018Spec>;
#[doc = "Field `REGCRQUEUESIZE` reader - REG_CR_QUEUE_SIZE"]
pub type RegcrqueuesizeR = crate::FieldReader;
#[doc = "Field `REGIBISTATUSSIZE` reader - REG_IBI_STATUS_SIZE"]
pub type RegibistatussizeR = crate::FieldReader;
#[doc = "Field `REGRXDATABUFFERSIZE` reader - REG_RX_DATA_BUFFER_SIZE"]
pub type RegrxdatabuffersizeR = crate::FieldReader;
#[doc = "Field `REGTXDATABUFFERSIZE` reader - REG_TX_DATA_BUFFER_SIZE"]
pub type RegtxdatabuffersizeR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - REG_CR_QUEUE_SIZE"]
    #[inline(always)]
    pub fn regcrqueuesize(&self) -> RegcrqueuesizeR {
        RegcrqueuesizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - REG_IBI_STATUS_SIZE"]
    #[inline(always)]
    pub fn regibistatussize(&self) -> RegibistatussizeR {
        RegibistatussizeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_RX_DATA_BUFFER_SIZE"]
    #[inline(always)]
    pub fn regrxdatabuffersize(&self) -> RegrxdatabuffersizeR {
        RegrxdatabuffersizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REG_TX_DATA_BUFFER_SIZE"]
    #[inline(always)]
    pub fn regtxdatabuffersize(&self) -> RegtxdatabuffersizeR {
        RegtxdatabuffersizeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {}
#[doc = "QUEUE\\_SIZE\n\nYou can [`read`](crate::Reg::read) this register and get [`hcipio018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcipio018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hcipio018Spec;
impl crate::RegisterSpec for Hcipio018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcipio018::R`](R) reader structure"]
impl crate::Readable for Hcipio018Spec {}
#[doc = "`write(|w| ..)` method takes [`hcipio018::W`](W) writer structure"]
impl crate::Writable for Hcipio018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HCIPIO018 to value 0x0404_2004"]
impl crate::Resettable for Hcipio018Spec {
    const RESET_VALUE: u32 = 0x0404_2004;
}
