#[doc = "Register `VIC80C` reader"]
pub type R = crate::R<Vic80cSpec>;
#[doc = "Register `VIC80C` writer"]
pub type W = crate::W<Vic80cSpec>;
#[doc = "Field `VICMCUINTREVT3` reader - VIC_MCU_INTR_EVT_3"]
pub type Vicmcuintrevt3R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREVT3` writer - VIC_MCU_INTR_EVT_3"]
pub type Vicmcuintrevt3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_3"]
    #[inline(always)]
    pub fn vicmcuintrevt3(&self) -> Vicmcuintrevt3R {
        Vicmcuintrevt3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EVT_3"]
    #[inline(always)]
    pub fn vicmcuintrevt3(&mut self) -> Vicmcuintrevt3W<Vic80cSpec> {
        Vicmcuintrevt3W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic80c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic80c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic80cSpec;
impl crate::RegisterSpec for Vic80cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic80c::R`](R) reader structure"]
impl crate::Readable for Vic80cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic80c::W`](W) writer structure"]
impl crate::Writable for Vic80cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC80C to value 0"]
impl crate::Resettable for Vic80cSpec {}
