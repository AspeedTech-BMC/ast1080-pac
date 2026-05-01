#[doc = "Register `VIC89C` reader"]
pub type R = crate::R<Vic89cSpec>;
#[doc = "Register `VIC89C` writer"]
pub type W = crate::W<Vic89cSpec>;
#[doc = "Field `VICMCUINTREN7` reader - VIC_MCU_INTR_EN_7"]
pub type Vicmcuintren7R = crate::FieldReader<u16>;
#[doc = "Field `VICMCUINTREN7` writer - VIC_MCU_INTR_EN_7"]
pub type Vicmcuintren7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VIC_MCU_INTR_EN_7"]
    #[inline(always)]
    pub fn vicmcuintren7(&self) -> Vicmcuintren7R {
        Vicmcuintren7R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VIC_MCU_INTR_EN_7"]
    #[inline(always)]
    pub fn vicmcuintren7(&mut self) -> Vicmcuintren7W<Vic89cSpec> {
        Vicmcuintren7W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Enable 7\n\nYou can [`read`](crate::Reg::read) this register and get [`vic89c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic89c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic89cSpec;
impl crate::RegisterSpec for Vic89cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic89c::R`](R) reader structure"]
impl crate::Readable for Vic89cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic89c::W`](W) writer structure"]
impl crate::Writable for Vic89cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC89C to value 0"]
impl crate::Resettable for Vic89cSpec {}
