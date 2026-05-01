#[doc = "Register `SCU0C0` reader"]
pub type R = crate::R<Scu0c0Spec>;
#[doc = "Register `SCU0C0` writer"]
pub type W = crate::W<Scu0c0Spec>;
#[doc = "Field `SCUJTAGMODE` reader - SCU_JTAG_MODE"]
pub type ScujtagmodeR = crate::FieldReader;
#[doc = "Field `SCUJTAGMODE` writer - SCU_JTAG_MODE"]
pub type ScujtagmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUGPIOFIQSEL` reader - SCU_GPIO_FIQ_SEL"]
pub type ScugpiofiqselR = crate::FieldReader;
#[doc = "Field `SCUGPIOFIQSEL` writer - SCU_GPIO_FIQ_SEL"]
pub type ScugpiofiqselW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUGPIOFIQEN` reader - SCU_GPIO_FIQ_EN"]
pub type ScugpiofiqenR = crate::BitReader;
#[doc = "Field `SCUGPIOFIQEN` writer - SCU_GPIO_FIQ_EN"]
pub type ScugpiofiqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUUARTBMCSEL` reader - SCU_UARTBMC_SEL"]
pub type ScuuartbmcselR = crate::FieldReader;
#[doc = "Field `SCUUARTBMCSEL` writer - SCU_UARTBMC_SEL"]
pub type ScuuartbmcselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SCUUARTDBGSEL` reader - SCU_UARTDBG_SEL"]
pub type ScuuartdbgselR = crate::BitReader;
#[doc = "Field `SCUUARTDBGSEL` writer - SCU_UARTDBG_SEL"]
pub type ScuuartdbgselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUBYPASSRESET` reader - SCU_BYPASS_RESET"]
pub type ScubypassresetR = crate::BitReader;
#[doc = "Field `SCUBYPASSRESET` writer - SCU_BYPASS_RESET"]
pub type ScubypassresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 4:5 - SCU_JTAG_MODE"]
    #[inline(always)]
    pub fn scujtagmode(&self) -> ScujtagmodeR {
        ScujtagmodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:13 - SCU_GPIO_FIQ_SEL"]
    #[inline(always)]
    pub fn scugpiofiqsel(&self) -> ScugpiofiqselR {
        ScugpiofiqselR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_GPIO_FIQ_EN"]
    #[inline(always)]
    pub fn scugpiofiqen(&self) -> ScugpiofiqenR {
        ScugpiofiqenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - SCU_UARTBMC_SEL"]
    #[inline(always)]
    pub fn scuuartbmcsel(&self) -> ScuuartbmcselR {
        ScuuartbmcselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - SCU_UARTDBG_SEL"]
    #[inline(always)]
    pub fn scuuartdbgsel(&self) -> ScuuartdbgselR {
        ScuuartdbgselR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - SCU_BYPASS_RESET"]
    #[inline(always)]
    pub fn scubypassreset(&self) -> ScubypassresetR {
        ScubypassresetR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 4:5 - SCU_JTAG_MODE"]
    #[inline(always)]
    pub fn scujtagmode(&mut self) -> ScujtagmodeW<Scu0c0Spec> {
        ScujtagmodeW::new(self, 4)
    }
    #[doc = "Bits 8:13 - SCU_GPIO_FIQ_SEL"]
    #[inline(always)]
    pub fn scugpiofiqsel(&mut self) -> ScugpiofiqselW<Scu0c0Spec> {
        ScugpiofiqselW::new(self, 8)
    }
    #[doc = "Bit 15 - SCU_GPIO_FIQ_EN"]
    #[inline(always)]
    pub fn scugpiofiqen(&mut self) -> ScugpiofiqenW<Scu0c0Spec> {
        ScugpiofiqenW::new(self, 15)
    }
    #[doc = "Bits 20:21 - SCU_UARTBMC_SEL"]
    #[inline(always)]
    pub fn scuuartbmcsel(&mut self) -> ScuuartbmcselW<Scu0c0Spec> {
        ScuuartbmcselW::new(self, 20)
    }
    #[doc = "Bit 22 - SCU_UARTDBG_SEL"]
    #[inline(always)]
    pub fn scuuartdbgsel(&mut self) -> ScuuartdbgselW<Scu0c0Spec> {
        ScuuartdbgselW::new(self, 22)
    }
    #[doc = "Bit 31 - SCU_BYPASS_RESET"]
    #[inline(always)]
    pub fn scubypassreset(&mut self) -> ScubypassresetW<Scu0c0Spec> {
        ScubypassresetW::new(self, 31)
    }
}
#[doc = "Misc. Control Set 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0c0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0c0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0c0Spec;
impl crate::RegisterSpec for Scu0c0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0c0::R`](R) reader structure"]
impl crate::Readable for Scu0c0Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0c0::W`](W) writer structure"]
impl crate::Writable for Scu0c0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0C0 to value 0x0010_0000"]
impl crate::Resettable for Scu0c0Spec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
