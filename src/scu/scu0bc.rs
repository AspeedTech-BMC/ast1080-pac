#[doc = "Register `SCU0BC` reader"]
pub type R = crate::R<Scu0bcSpec>;
#[doc = "Register `SCU0BC` writer"]
pub type W = crate::W<Scu0bcSpec>;
#[doc = "Field `SCUDBGNORMALRATE` reader - SCU_DBG_NORMAL_RATE"]
pub type ScudbgnormalrateR = crate::FieldReader<u16>;
#[doc = "Field `SCUDBGNORMALRATE` writer - SCU_DBG_NORMAL_RATE"]
pub type ScudbgnormalrateW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `SCUDBGPASSRATE` reader - SCU_DBG_PASS_RATE"]
pub type ScudbgpassrateR = crate::FieldReader<u16>;
#[doc = "Field `SCUDBGPASSRATE` writer - SCU_DBG_PASS_RATE"]
pub type ScudbgpassrateW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - SCU_DBG_NORMAL_RATE"]
    #[inline(always)]
    pub fn scudbgnormalrate(&self) -> ScudbgnormalrateR {
        ScudbgnormalrateR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SCU_DBG_PASS_RATE"]
    #[inline(always)]
    pub fn scudbgpassrate(&self) -> ScudbgpassrateR {
        ScudbgpassrateR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - SCU_DBG_NORMAL_RATE"]
    #[inline(always)]
    pub fn scudbgnormalrate(&mut self) -> ScudbgnormalrateW<Scu0bcSpec> {
        ScudbgnormalrateW::new(self, 0)
    }
    #[doc = "Bits 16:31 - SCU_DBG_PASS_RATE"]
    #[inline(always)]
    pub fn scudbgpassrate(&mut self) -> ScudbgpassrateW<Scu0bcSpec> {
        ScudbgpassrateW::new(self, 16)
    }
}
#[doc = "Debug UART Baudrate\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0bc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0bc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0bcSpec;
impl crate::RegisterSpec for Scu0bcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0bc::R`](R) reader structure"]
impl crate::Readable for Scu0bcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu0bc::W`](W) writer structure"]
impl crate::Writable for Scu0bcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0BC to value 0x0060_0001"]
impl crate::Resettable for Scu0bcSpec {
    const RESET_VALUE: u32 = 0x0060_0001;
}
