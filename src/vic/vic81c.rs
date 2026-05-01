#[doc = "Register `VIC81C` reader"]
pub type R = crate::R<Vic81cSpec>;
#[doc = "Register `VIC81C` writer"]
pub type W = crate::W<Vic81cSpec>;
#[doc = "Field `VICMCUINTREVT7` reader - VIC_MCU_INTR_EVT_7"]
pub type Vicmcuintrevt7R = crate::FieldReader<u16>;
#[doc = "Field `VICMCUINTREVT7` writer - VIC_MCU_INTR_EVT_7"]
pub type Vicmcuintrevt7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VIC_MCU_INTR_EVT_7"]
    #[inline(always)]
    pub fn vicmcuintrevt7(&self) -> Vicmcuintrevt7R {
        Vicmcuintrevt7R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VIC_MCU_INTR_EVT_7"]
    #[inline(always)]
    pub fn vicmcuintrevt7(&mut self) -> Vicmcuintrevt7W<Vic81cSpec> {
        Vicmcuintrevt7W::new(self, 0)
    }
}
#[doc = "MCU Interrupt Event 7\n\nYou can [`read`](crate::Reg::read) this register and get [`vic81c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic81c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic81cSpec;
impl crate::RegisterSpec for Vic81cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic81c::R`](R) reader structure"]
impl crate::Readable for Vic81cSpec {}
#[doc = "`write(|w| ..)` method takes [`vic81c::W`](W) writer structure"]
impl crate::Writable for Vic81cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC81C to value 0"]
impl crate::Resettable for Vic81cSpec {}
