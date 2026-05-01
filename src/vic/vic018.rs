#[doc = "Register `VIC018` reader"]
pub type R = crate::R<Vic018Spec>;
#[doc = "Register `VIC018` writer"]
pub type W = crate::W<Vic018Spec>;
#[doc = "Field `VICPSPSWINTTOSSMCU` reader - VIC_PSP_SW_INT_TO_SSMCU"]
pub type VicpspswinttossmcuR = crate::FieldReader;
#[doc = "Field `VICPSPSWINTTOSSMCU` writer - VIC_PSP_SW_INT_TO_SSMCU"]
pub type VicpspswinttossmcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `VICPSPSWINTTOMCU` reader - VIC_PSP_SW_INT_TO_MCU"]
pub type VicpspswinttomcuR = crate::FieldReader;
#[doc = "Field `VICPSPSWINTTOMCU` writer - VIC_PSP_SW_INT_TO_MCU"]
pub type VicpspswinttomcuW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
impl R {
    #[doc = "Bits 0:6 - VIC_PSP_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicpspswinttossmcu(&self) -> VicpspswinttossmcuR {
        VicpspswinttossmcuR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - VIC_PSP_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicpspswinttomcu(&self) -> VicpspswinttomcuR {
        VicpspswinttomcuR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - VIC_PSP_SW_INT_TO_SSMCU"]
    #[inline(always)]
    pub fn vicpspswinttossmcu(&mut self) -> VicpspswinttossmcuW<Vic018Spec> {
        VicpspswinttossmcuW::new(self, 0)
    }
    #[doc = "Bits 8:14 - VIC_PSP_SW_INT_TO_MCU"]
    #[inline(always)]
    pub fn vicpspswinttomcu(&mut self) -> VicpspswinttomcuW<Vic018Spec> {
        VicpspswinttomcuW::new(self, 8)
    }
}
#[doc = "PSP Software Interrupt Set\n\nYou can [`read`](crate::Reg::read) this register and get [`vic018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic018Spec;
impl crate::RegisterSpec for Vic018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic018::R`](R) reader structure"]
impl crate::Readable for Vic018Spec {}
#[doc = "`write(|w| ..)` method takes [`vic018::W`](W) writer structure"]
impl crate::Writable for Vic018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC018 to value 0"]
impl crate::Resettable for Vic018Spec {}
