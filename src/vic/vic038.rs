#[doc = "Register `VIC038` reader"]
pub type R = crate::R<Vic038Spec>;
#[doc = "Register `VIC038` writer"]
pub type W = crate::W<Vic038Spec>;
#[doc = "Field `VICMCUSWINTTOPSP` reader - VIC_MCU_SW_INT_TO_PSP"]
pub type VicmcuswinttopspR = crate::FieldReader;
#[doc = "Field `VICMCUSWINTTOPSP` writer - VIC_MCU_SW_INT_TO_PSP"]
pub type VicmcuswinttopspW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `VICMCUSWINTTOSSMCU` reader - VIC_MCU_SW_INT_TO_SSMCU"]
pub type VicmcuswinttossmcuR = crate::FieldReader;
#[doc = "Field `VICMCUSWINTTOSSMCU` writer - VIC_MCU_SW_INT_TO_SSMCU"]
pub type VicmcuswinttossmcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - VIC_MCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicmcuswinttopsp(&self) -> VicmcuswinttopspR {
        VicmcuswinttopspR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_MCU_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicmcuswinttossmcu(&self) -> VicmcuswinttossmcuR {
        VicmcuswinttossmcuR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - VIC_MCU_SW_INT_TO_PSP"]
    #[inline(always)]
    pub fn vicmcuswinttopsp(&mut self) -> VicmcuswinttopspW<Vic038Spec> {
        VicmcuswinttopspW::new(self, 0)
    }
    #[doc = "Bits 8:14 - VIC_MCU_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicmcuswinttossmcu(&mut self) -> VicmcuswinttossmcuW<Vic038Spec> {
        VicmcuswinttossmcuW::new(self, 8)
    }
}
#[doc = "MCU Software Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`vic038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic038Spec;
impl crate::RegisterSpec for Vic038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic038::R`](R) reader structure"]
impl crate::Readable for Vic038Spec {}
#[doc = "`write(|w| ..)` method takes [`vic038::W`](W) writer structure"]
impl crate::Writable for Vic038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC038 to value 0"]
impl crate::Resettable for Vic038Spec {}
