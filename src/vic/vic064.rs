#[doc = "Register `VIC064` reader"]
pub type R = crate::R<Vic064Spec>;
#[doc = "Register `VIC064` writer"]
pub type W = crate::W<Vic064Spec>;
#[doc = "Field `VICHBOE` reader - VIC_HB_OE"]
pub type VichboeR = crate::BitReader;
#[doc = "Field `VICHBOE` writer - VIC_HB_OE"]
pub type VichboeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - VIC_HB_OE"]
    #[inline(always)]
    pub fn vichboe(&self) -> VichboeR {
        VichboeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VIC_HB_OE"]
    #[inline(always)]
    pub fn vichboe(&mut self) -> VichboeW<Vic064Spec> {
        VichboeW::new(self, 0)
    }
}
#[doc = "HeartBeat Output Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`vic064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic064Spec;
impl crate::RegisterSpec for Vic064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic064::R`](R) reader structure"]
impl crate::Readable for Vic064Spec {}
#[doc = "`write(|w| ..)` method takes [`vic064::W`](W) writer structure"]
impl crate::Writable for Vic064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC064 to value 0"]
impl crate::Resettable for Vic064Spec {}
