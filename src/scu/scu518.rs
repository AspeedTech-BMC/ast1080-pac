#[doc = "Register `SCU518` reader"]
pub type R = crate::R<Scu518Spec>;
#[doc = "Register `SCU518` writer"]
pub type W = crate::W<Scu518Spec>;
#[doc = "Field `SCUDISPDIO076` reader - SCU_DIS_PD_IO076"]
pub type Scudispdio076R = crate::BitReader;
#[doc = "Field `SCUDISPDIO076` writer - SCU_DIS_PD_IO076"]
pub type Scudispdio076W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO076` reader - SCU_DIS_PU_IO076"]
pub type Scudispuio076R = crate::BitReader;
#[doc = "Field `SCUDISPUIO076` writer - SCU_DIS_PU_IO076"]
pub type Scudispuio076W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO076` reader - SCU_DRV_IO076"]
pub type Scudrvio076R = crate::FieldReader;
#[doc = "Field `SCUDRVIO076` writer - SCU_DRV_IO076"]
pub type Scudrvio076W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO076` reader - SCU_EN_SMT_IO076"]
pub type Scuensmtio076R = crate::BitReader;
#[doc = "Field `SCUENSMTIO076` writer - SCU_EN_SMT_IO076"]
pub type Scuensmtio076W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO076` reader - SCU_EN_HV_IO076"]
pub type Scuenhvio076R = crate::BitReader;
#[doc = "Field `SCUENHVIO076` writer - SCU_EN_HV_IO076"]
pub type Scuenhvio076W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO077` reader - SCU_DIS_PD_IO077"]
pub type Scudispdio077R = crate::BitReader;
#[doc = "Field `SCUDISPDIO077` writer - SCU_DIS_PD_IO077"]
pub type Scudispdio077W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO077` reader - SCU_DIS_PU_IO077"]
pub type Scudispuio077R = crate::BitReader;
#[doc = "Field `SCUDISPUIO077` writer - SCU_DIS_PU_IO077"]
pub type Scudispuio077W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO077` reader - SCU_DRV_IO077"]
pub type Scudrvio077R = crate::FieldReader;
#[doc = "Field `SCUDRVIO077` writer - SCU_DRV_IO077"]
pub type Scudrvio077W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO077` reader - SCU_EN_SMT_IO077"]
pub type Scuensmtio077R = crate::BitReader;
#[doc = "Field `SCUENSMTIO077` writer - SCU_EN_SMT_IO077"]
pub type Scuensmtio077W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO077` reader - SCU_EN_HV_IO077"]
pub type Scuenhvio077R = crate::BitReader;
#[doc = "Field `SCUENHVIO077` writer - SCU_EN_HV_IO077"]
pub type Scuenhvio077W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO076"]
    #[inline(always)]
    pub fn scudispdio076(&self) -> Scudispdio076R {
        Scudispdio076R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO076"]
    #[inline(always)]
    pub fn scudispuio076(&self) -> Scudispuio076R {
        Scudispuio076R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO076"]
    #[inline(always)]
    pub fn scudrvio076(&self) -> Scudrvio076R {
        Scudrvio076R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO076"]
    #[inline(always)]
    pub fn scuensmtio076(&self) -> Scuensmtio076R {
        Scuensmtio076R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO076"]
    #[inline(always)]
    pub fn scuenhvio076(&self) -> Scuenhvio076R {
        Scuenhvio076R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO077"]
    #[inline(always)]
    pub fn scudispdio077(&self) -> Scudispdio077R {
        Scudispdio077R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO077"]
    #[inline(always)]
    pub fn scudispuio077(&self) -> Scudispuio077R {
        Scudispuio077R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO077"]
    #[inline(always)]
    pub fn scudrvio077(&self) -> Scudrvio077R {
        Scudrvio077R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO077"]
    #[inline(always)]
    pub fn scuensmtio077(&self) -> Scuensmtio077R {
        Scuensmtio077R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO077"]
    #[inline(always)]
    pub fn scuenhvio077(&self) -> Scuenhvio077R {
        Scuenhvio077R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO076"]
    #[inline(always)]
    pub fn scudispdio076(&mut self) -> Scudispdio076W<Scu518Spec> {
        Scudispdio076W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO076"]
    #[inline(always)]
    pub fn scudispuio076(&mut self) -> Scudispuio076W<Scu518Spec> {
        Scudispuio076W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO076"]
    #[inline(always)]
    pub fn scudrvio076(&mut self) -> Scudrvio076W<Scu518Spec> {
        Scudrvio076W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO076"]
    #[inline(always)]
    pub fn scuensmtio076(&mut self) -> Scuensmtio076W<Scu518Spec> {
        Scuensmtio076W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO076"]
    #[inline(always)]
    pub fn scuenhvio076(&mut self) -> Scuenhvio076W<Scu518Spec> {
        Scuenhvio076W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO077"]
    #[inline(always)]
    pub fn scudispdio077(&mut self) -> Scudispdio077W<Scu518Spec> {
        Scudispdio077W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO077"]
    #[inline(always)]
    pub fn scudispuio077(&mut self) -> Scudispuio077W<Scu518Spec> {
        Scudispuio077W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO077"]
    #[inline(always)]
    pub fn scudrvio077(&mut self) -> Scudrvio077W<Scu518Spec> {
        Scudrvio077W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO077"]
    #[inline(always)]
    pub fn scuensmtio077(&mut self) -> Scuensmtio077W<Scu518Spec> {
        Scuensmtio077W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO077"]
    #[inline(always)]
    pub fn scuenhvio077(&mut self) -> Scuenhvio077W<Scu518Spec> {
        Scuenhvio077W::new(self, 25)
    }
}
#[doc = "IO Control \\#39\n\nYou can [`read`](crate::Reg::read) this register and get [`scu518::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu518::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu518Spec;
impl crate::RegisterSpec for Scu518Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu518::R`](R) reader structure"]
impl crate::Readable for Scu518Spec {}
#[doc = "`write(|w| ..)` method takes [`scu518::W`](W) writer structure"]
impl crate::Writable for Scu518Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU518 to value 0x0204_0204"]
impl crate::Resettable for Scu518Spec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
