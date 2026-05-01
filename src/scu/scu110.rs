#[doc = "Register `SCU110` reader"]
pub type R = crate::R<Scu110Spec>;
#[doc = "Register `SCU110` writer"]
pub type W = crate::W<Scu110Spec>;
#[doc = "Field `SCUDISEMMODE` reader - SCU_DIS_EM_MODE"]
pub type ScudisemmodeR = crate::BitReader;
#[doc = "Field `SCUDISEMMODE` writer - SCU_DIS_EM_MODE"]
pub type ScudisemmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader<u32>;
#[doc = "Field `SCUMCULOCK` reader - SCU_MCU_LOCK"]
pub type ScumculockR = crate::BitReader;
#[doc = "Field `SCUMCULOCK` writer - SCU_MCU_LOCK"]
pub type ScumculockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUMCUSLEEP` reader - SCU_MCU_SLEEP"]
pub type ScumcusleepR = crate::BitReader;
#[doc = "Field `SCUMCUMAJORALERT` reader - SCU_MCU_MAJOR_ALERT"]
pub type ScumcumajoralertR = crate::BitReader;
#[doc = "Field `SCUMCUMINORALERT` reader - SCU_MCU_MINOR_ALERT"]
pub type ScumcuminoralertR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - SCU_DIS_EM_MODE"]
    #[inline(always)]
    pub fn scudisemmode(&self) -> ScudisemmodeR {
        ScudisemmodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new((self.bits >> 2) & 0x003f_ffff)
    }
    #[doc = "Bit 24 - SCU_MCU_LOCK"]
    #[inline(always)]
    pub fn scumculock(&self) -> ScumculockR {
        ScumculockR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - SCU_MCU_SLEEP"]
    #[inline(always)]
    pub fn scumcusleep(&self) -> ScumcusleepR {
        ScumcusleepR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - SCU_MCU_MAJOR_ALERT"]
    #[inline(always)]
    pub fn scumcumajoralert(&self) -> ScumcumajoralertR {
        ScumcumajoralertR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_MCU_MINOR_ALERT"]
    #[inline(always)]
    pub fn scumcuminoralert(&self) -> ScumcuminoralertR {
        ScumcuminoralertR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SCU_DIS_EM_MODE"]
    #[inline(always)]
    pub fn scudisemmode(&mut self) -> ScudisemmodeW<Scu110Spec> {
        ScudisemmodeW::new(self, 1)
    }
    #[doc = "Bit 24 - SCU_MCU_LOCK"]
    #[inline(always)]
    pub fn scumculock(&mut self) -> ScumculockW<Scu110Spec> {
        ScumculockW::new(self, 24)
    }
}
#[doc = "MCU Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu110::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu110::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu110Spec;
impl crate::RegisterSpec for Scu110Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu110::R`](R) reader structure"]
impl crate::Readable for Scu110Spec {}
#[doc = "`write(|w| ..)` method takes [`scu110::W`](W) writer structure"]
impl crate::Writable for Scu110Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU110 to value 0"]
impl crate::Resettable for Scu110Spec {}
