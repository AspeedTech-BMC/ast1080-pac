#[doc = "Register `HUB44` reader"]
pub type R = crate::R<Hub44Spec>;
#[doc = "Register `HUB44` writer"]
pub type W = crate::W<Hub44Spec>;
#[doc = "Field `DMAToMemoryREQCounter` reader - DMA to Memory REQ counter"]
pub type DmatoMemoryReqcounterR = crate::FieldReader;
#[doc = "Field `DMAToMemoryACKCounter` reader - DMA to Memory ACK counter"]
pub type DmatoMemoryAckcounterR = crate::FieldReader;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:7 - DMA to Memory REQ counter"]
    #[inline(always)]
    pub fn dmato_memory_reqcounter(&self) -> DmatoMemoryReqcounterR {
        DmatoMemoryReqcounterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - DMA to Memory ACK counter"]
    #[inline(always)]
    pub fn dmato_memory_ackcounter(&self) -> DmatoMemoryAckcounterR {
        DmatoMemoryAckcounterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "DMA to Memory Synchronization Status\n\nYou can [`read`](crate::Reg::read) this register and get [`hub44::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub44::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub44Spec;
impl crate::RegisterSpec for Hub44Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub44::R`](R) reader structure"]
impl crate::Readable for Hub44Spec {}
#[doc = "`write(|w| ..)` method takes [`hub44::W`](W) writer structure"]
impl crate::Writable for Hub44Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB44 to value 0"]
impl crate::Resettable for Hub44Spec {}
