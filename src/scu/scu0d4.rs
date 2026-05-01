#[doc = "Register `SCU0D4` reader"]
pub type R = crate::R<Scu0d4Spec>;
#[doc = "Register `SCU0D4` writer"]
pub type W = crate::W<Scu0d4Spec>;
#[doc = "Field `SCUSPI0OMODE` reader - SCU_SPI0O_MODE"]
pub type Scuspi0omodeR = crate::BitReader;
#[doc = "Field `SCUSPI0OMODE` writer - SCU_SPI0O_MODE"]
pub type Scuspi0omodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSPI1OMODE` reader - SCU_SPI1O_MODE"]
pub type Scuspi1omodeR = crate::BitReader;
#[doc = "Field `SCUSPI1OMODE` writer - SCU_SPI1O_MODE"]
pub type Scuspi1omodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSPI2OMODE` reader - SCU_SPI2O_MODE"]
pub type Scuspi2omodeR = crate::BitReader;
#[doc = "Field `SCUSPI2OMODE` writer - SCU_SPI2O_MODE"]
pub type Scuspi2omodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUSPI3OMODE` reader - SCU_SPI3O_MODE"]
pub type Scuspi3omodeR = crate::BitReader;
#[doc = "Field `SCUSPI3OMODE` writer - SCU_SPI3O_MODE"]
pub type Scuspi3omodeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_SPI0O_MODE"]
    #[inline(always)]
    pub fn scuspi0omode(&self) -> Scuspi0omodeR {
        Scuspi0omodeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_SPI1O_MODE"]
    #[inline(always)]
    pub fn scuspi1omode(&self) -> Scuspi1omodeR {
        Scuspi1omodeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_SPI2O_MODE"]
    #[inline(always)]
    pub fn scuspi2omode(&self) -> Scuspi2omodeR {
        Scuspi2omodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SCU_SPI3O_MODE"]
    #[inline(always)]
    pub fn scuspi3omode(&self) -> Scuspi3omodeR {
        Scuspi3omodeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_SPI0O_MODE"]
    #[inline(always)]
    pub fn scuspi0omode(&mut self) -> Scuspi0omodeW<Scu0d4Spec> {
        Scuspi0omodeW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_SPI1O_MODE"]
    #[inline(always)]
    pub fn scuspi1omode(&mut self) -> Scuspi1omodeW<Scu0d4Spec> {
        Scuspi1omodeW::new(self, 1)
    }
    #[doc = "Bit 2 - SCU_SPI2O_MODE"]
    #[inline(always)]
    pub fn scuspi2omode(&mut self) -> Scuspi2omodeW<Scu0d4Spec> {
        Scuspi2omodeW::new(self, 2)
    }
    #[doc = "Bit 3 - SCU_SPI3O_MODE"]
    #[inline(always)]
    pub fn scuspi3omode(&mut self) -> Scuspi3omodeW<Scu0d4Spec> {
        Scuspi3omodeW::new(self, 3)
    }
}
#[doc = "SPI Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu0d4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu0d4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu0d4Spec;
impl crate::RegisterSpec for Scu0d4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu0d4::R`](R) reader structure"]
impl crate::Readable for Scu0d4Spec {}
#[doc = "`write(|w| ..)` method takes [`scu0d4::W`](W) writer structure"]
impl crate::Writable for Scu0d4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU0D4 to value 0x0f"]
impl crate::Resettable for Scu0d4Spec {
    const RESET_VALUE: u32 = 0x0f;
}
