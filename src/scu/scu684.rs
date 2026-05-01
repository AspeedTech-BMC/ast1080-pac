#[doc = "Register `SCU684` reader"]
pub type R = crate::R<Scu684Spec>;
#[doc = "Register `SCU684` writer"]
pub type W = crate::W<Scu684Spec>;
#[doc = "Field `SCUIOCTRLCLK` reader - SCU_IO_CTRL_CLK"]
pub type ScuioctrlclkR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLCLK` writer - SCU_IO_CTRL_CLK"]
pub type ScuioctrlclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLCS` reader - SCU_IO_CTRL_CS"]
pub type ScuioctrlcsR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLCS` writer - SCU_IO_CTRL_CS"]
pub type ScuioctrlcsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLRST` reader - SCU_IO_CTRL_RST"]
pub type ScuioctrlrstR = crate::FieldReader;
#[doc = "Field `SCUIOCTRLRST` writer - SCU_IO_CTRL_RST"]
pub type ScuioctrlrstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLRWDS0` reader - SCU_IO_CTRL_RWDS0"]
pub type Scuioctrlrwds0R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLRWDS0` writer - SCU_IO_CTRL_RWDS0"]
pub type Scuioctrlrwds0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUIOCTRLRWDS1` reader - SCU_IO_CTRL_RWDS1"]
pub type Scuioctrlrwds1R = crate::FieldReader;
#[doc = "Field `SCUIOCTRLRWDS1` writer - SCU_IO_CTRL_RWDS1"]
pub type Scuioctrlrwds1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_CLK"]
    #[inline(always)]
    pub fn scuioctrlclk(&self) -> ScuioctrlclkR {
        ScuioctrlclkR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_CS"]
    #[inline(always)]
    pub fn scuioctrlcs(&self) -> ScuioctrlcsR {
        ScuioctrlcsR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_RST"]
    #[inline(always)]
    pub fn scuioctrlrst(&self) -> ScuioctrlrstR {
        ScuioctrlrstR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_RWDS0"]
    #[inline(always)]
    pub fn scuioctrlrwds0(&self) -> Scuioctrlrwds0R {
        Scuioctrlrwds0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SCU_IO_CTRL_RWDS1"]
    #[inline(always)]
    pub fn scuioctrlrwds1(&self) -> Scuioctrlrwds1R {
        Scuioctrlrwds1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SCU_IO_CTRL_CLK"]
    #[inline(always)]
    pub fn scuioctrlclk(&mut self) -> ScuioctrlclkW<Scu684Spec> {
        ScuioctrlclkW::new(self, 0)
    }
    #[doc = "Bits 4:7 - SCU_IO_CTRL_CS"]
    #[inline(always)]
    pub fn scuioctrlcs(&mut self) -> ScuioctrlcsW<Scu684Spec> {
        ScuioctrlcsW::new(self, 4)
    }
    #[doc = "Bits 8:11 - SCU_IO_CTRL_RST"]
    #[inline(always)]
    pub fn scuioctrlrst(&mut self) -> ScuioctrlrstW<Scu684Spec> {
        ScuioctrlrstW::new(self, 8)
    }
    #[doc = "Bits 12:15 - SCU_IO_CTRL_RWDS0"]
    #[inline(always)]
    pub fn scuioctrlrwds0(&mut self) -> Scuioctrlrwds0W<Scu684Spec> {
        Scuioctrlrwds0W::new(self, 12)
    }
    #[doc = "Bits 16:19 - SCU_IO_CTRL_RWDS1"]
    #[inline(always)]
    pub fn scuioctrlrwds1(&mut self) -> Scuioctrlrwds1W<Scu684Spec> {
        Scuioctrlrwds1W::new(self, 16)
    }
}
#[doc = "HRAM IO Control 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu684::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu684::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu684Spec;
impl crate::RegisterSpec for Scu684Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu684::R`](R) reader structure"]
impl crate::Readable for Scu684Spec {}
#[doc = "`write(|w| ..)` method takes [`scu684::W`](W) writer structure"]
impl crate::Writable for Scu684Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU684 to value 0x0001_1331"]
impl crate::Resettable for Scu684Spec {
    const RESET_VALUE: u32 = 0x0001_1331;
}
