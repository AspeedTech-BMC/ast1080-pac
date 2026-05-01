#[doc = "Register `VIC88C` reader"]
pub type R = crate::R<Vic88cSpec>;
#[doc = "Register `VIC88C` writer"]
pub type W = crate::W<Vic88cSpec>;
#[doc = "Field `VICMCUINTREN3` reader - VIC_MCU_INTR_EN_3"]
pub type Vicmcuintren3R = crate::FieldReader<u32>;
#[doc = "Field `VICMCUINTREN3` writer - VIC_MCU_INTR_EN_3"]
pub type Vicmcuintren3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_3"]
    #[inline(always)]
    pub fn vicmcuintren3(&self) -> Vicmcuintren3R {
        Vicmcuintren3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - VIC_MCU_INTR_EN_3"]
    #[inline(always)]
    pub fn vicmcuintren3(&mut self) -> Vicmcuintren3W<Vic88cSpec> {
        Vicmcuintren3W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 3\n\nYou can [`read`](crate::Reg::read) this register and get [`vic88c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic88c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic88cSpec;
impl crate::RegisterSpec for Vic88cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic88c::R`](R) reader structure"]
impl crate::Readable for Vic88cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic88c::W`](W) writer structure"]
impl crate::Writable for Vic88cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC88C to value 0"]
impl crate::Resettable for Vic88cSpec {}
