#[doc = "Register `HUB04` reader"]
pub type R = crate::R<Hub04Spec>;
#[doc = "Register `HUB04` writer"]
pub type W = crate::W<Hub04Spec>;
#[doc = "Field `RootFnDevAddr` reader - Root function device address"]
pub type RootFnDevAddrR = crate::FieldReader;
#[doc = "Field `RootFnDevAddr` writer - Root function device address"]
pub type RootFnDevAddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `StatusOfDMAPageBufRegdebug` reader - Status of DMA page buffer regdebug"]
pub type StatusOfDmapageBufRegdebugR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:6 - Root function device address"]
    #[inline(always)]
    pub fn root_fn_dev_addr(&self) -> RootFnDevAddrR {
        RootFnDevAddrR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:23 - Status of DMA page buffer regdebug"]
    #[inline(always)]
    pub fn status_of_dmapage_buf_regdebug(&self) -> StatusOfDmapageBufRegdebugR {
        StatusOfDmapageBufRegdebugR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Root function device address"]
    #[inline(always)]
    pub fn root_fn_dev_addr(&mut self) -> RootFnDevAddrW<Hub04Spec> {
        RootFnDevAddrW::new(self, 0)
    }
}
#[doc = "Root Configuration Setting Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub04::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub04::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub04Spec;
impl crate::RegisterSpec for Hub04Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub04::R`](R) reader structure"]
impl crate::Readable for Hub04Spec {}
#[doc = "`write(|w| ..)` method takes [`hub04::W`](W) writer structure"]
impl crate::Writable for Hub04Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB04 to value 0"]
impl crate::Resettable for Hub04Spec {}
