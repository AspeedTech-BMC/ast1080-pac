#[doc = "Register `SCU514` reader"]
pub type R = crate::R<Scu514Spec>;
#[doc = "Register `SCU514` writer"]
pub type W = crate::W<Scu514Spec>;
#[doc = "Field `SCUDISPDIO074` reader - SCU_DIS_PD_IO074"]
pub type Scudispdio074R = crate::BitReader;
#[doc = "Field `SCUDISPDIO074` writer - SCU_DIS_PD_IO074"]
pub type Scudispdio074W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO074` reader - SCU_DIS_PU_IO074"]
pub type Scudispuio074R = crate::BitReader;
#[doc = "Field `SCUDISPUIO074` writer - SCU_DIS_PU_IO074"]
pub type Scudispuio074W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO074` reader - SCU_DRV_IO074"]
pub type Scudrvio074R = crate::FieldReader;
#[doc = "Field `SCUDRVIO074` writer - SCU_DRV_IO074"]
pub type Scudrvio074W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO074` reader - SCU_EN_SMT_IO074"]
pub type Scuensmtio074R = crate::BitReader;
#[doc = "Field `SCUENSMTIO074` writer - SCU_EN_SMT_IO074"]
pub type Scuensmtio074W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO074` reader - SCU_EN_HV_IO074"]
pub type Scuenhvio074R = crate::BitReader;
#[doc = "Field `SCUENHVIO074` writer - SCU_EN_HV_IO074"]
pub type Scuenhvio074W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO075` reader - SCU_DIS_PD_IO075"]
pub type Scudispdio075R = crate::BitReader;
#[doc = "Field `SCUDISPDIO075` writer - SCU_DIS_PD_IO075"]
pub type Scudispdio075W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO075` reader - SCU_DIS_PU_IO075"]
pub type Scudispuio075R = crate::BitReader;
#[doc = "Field `SCUDISPUIO075` writer - SCU_DIS_PU_IO075"]
pub type Scudispuio075W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO075` reader - SCU_DRV_IO075"]
pub type Scudrvio075R = crate::FieldReader;
#[doc = "Field `SCUDRVIO075` writer - SCU_DRV_IO075"]
pub type Scudrvio075W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO075` reader - SCU_EN_SMT_IO075"]
pub type Scuensmtio075R = crate::BitReader;
#[doc = "Field `SCUENSMTIO075` writer - SCU_EN_SMT_IO075"]
pub type Scuensmtio075W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO075` reader - SCU_EN_HV_IO075"]
pub type Scuenhvio075R = crate::BitReader;
#[doc = "Field `SCUENHVIO075` writer - SCU_EN_HV_IO075"]
pub type Scuenhvio075W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO074"]
    #[inline(always)]
    pub fn scudispdio074(&self) -> Scudispdio074R {
        Scudispdio074R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO074"]
    #[inline(always)]
    pub fn scudispuio074(&self) -> Scudispuio074R {
        Scudispuio074R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO074"]
    #[inline(always)]
    pub fn scudrvio074(&self) -> Scudrvio074R {
        Scudrvio074R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO074"]
    #[inline(always)]
    pub fn scuensmtio074(&self) -> Scuensmtio074R {
        Scuensmtio074R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO074"]
    #[inline(always)]
    pub fn scuenhvio074(&self) -> Scuenhvio074R {
        Scuenhvio074R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO075"]
    #[inline(always)]
    pub fn scudispdio075(&self) -> Scudispdio075R {
        Scudispdio075R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO075"]
    #[inline(always)]
    pub fn scudispuio075(&self) -> Scudispuio075R {
        Scudispuio075R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO075"]
    #[inline(always)]
    pub fn scudrvio075(&self) -> Scudrvio075R {
        Scudrvio075R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO075"]
    #[inline(always)]
    pub fn scuensmtio075(&self) -> Scuensmtio075R {
        Scuensmtio075R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO075"]
    #[inline(always)]
    pub fn scuenhvio075(&self) -> Scuenhvio075R {
        Scuenhvio075R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO074"]
    #[inline(always)]
    pub fn scudispdio074(&mut self) -> Scudispdio074W<Scu514Spec> {
        Scudispdio074W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO074"]
    #[inline(always)]
    pub fn scudispuio074(&mut self) -> Scudispuio074W<Scu514Spec> {
        Scudispuio074W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO074"]
    #[inline(always)]
    pub fn scudrvio074(&mut self) -> Scudrvio074W<Scu514Spec> {
        Scudrvio074W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO074"]
    #[inline(always)]
    pub fn scuensmtio074(&mut self) -> Scuensmtio074W<Scu514Spec> {
        Scuensmtio074W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO074"]
    #[inline(always)]
    pub fn scuenhvio074(&mut self) -> Scuenhvio074W<Scu514Spec> {
        Scuenhvio074W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO075"]
    #[inline(always)]
    pub fn scudispdio075(&mut self) -> Scudispdio075W<Scu514Spec> {
        Scudispdio075W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO075"]
    #[inline(always)]
    pub fn scudispuio075(&mut self) -> Scudispuio075W<Scu514Spec> {
        Scudispuio075W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO075"]
    #[inline(always)]
    pub fn scudrvio075(&mut self) -> Scudrvio075W<Scu514Spec> {
        Scudrvio075W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO075"]
    #[inline(always)]
    pub fn scuensmtio075(&mut self) -> Scuensmtio075W<Scu514Spec> {
        Scuensmtio075W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO075"]
    #[inline(always)]
    pub fn scuenhvio075(&mut self) -> Scuenhvio075W<Scu514Spec> {
        Scuenhvio075W::new(self, 25)
    }
}
#[doc = "IO Control \\#38\n\nYou can [`read`](crate::Reg::read) this register and get [`scu514::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu514::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu514Spec;
impl crate::RegisterSpec for Scu514Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu514::R`](R) reader structure"]
impl crate::Readable for Scu514Spec {}
#[doc = "`write(|w| ..)` method takes [`scu514::W`](W) writer structure"]
impl crate::Writable for Scu514Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU514 to value 0x0204_0204"]
impl crate::Resettable for Scu514Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
