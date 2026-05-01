#[doc = "Register `SCU4FC` reader"]
pub type R = crate::R<Scu4fcSpec>;
#[doc = "Register `SCU4FC` writer"]
pub type W = crate::W<Scu4fcSpec>;
#[doc = "Field `SCUDISPDIO062` reader - SCU_DIS_PD_IO062"]
pub type Scudispdio062R = crate::BitReader;
#[doc = "Field `SCUDISPDIO062` writer - SCU_DIS_PD_IO062"]
pub type Scudispdio062W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO062` reader - SCU_DIS_PU_IO062"]
pub type Scudispuio062R = crate::BitReader;
#[doc = "Field `SCUDISPUIO062` writer - SCU_DIS_PU_IO062"]
pub type Scudispuio062W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO062` reader - SCU_DRV_IO062"]
pub type Scudrvio062R = crate::FieldReader;
#[doc = "Field `SCUDRVIO062` writer - SCU_DRV_IO062"]
pub type Scudrvio062W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO062` reader - SCU_EN_SMT_IO062"]
pub type Scuensmtio062R = crate::BitReader;
#[doc = "Field `SCUENSMTIO062` writer - SCU_EN_SMT_IO062"]
pub type Scuensmtio062W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO062` reader - SCU_EN_HV_IO062"]
pub type Scuenhvio062R = crate::BitReader;
#[doc = "Field `SCUENHVIO062` writer - SCU_EN_HV_IO062"]
pub type Scuenhvio062W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO063` reader - SCU_DIS_PD_IO063"]
pub type Scudispdio063R = crate::BitReader;
#[doc = "Field `SCUDISPDIO063` writer - SCU_DIS_PD_IO063"]
pub type Scudispdio063W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO063` reader - SCU_DIS_PU_IO063"]
pub type Scudispuio063R = crate::BitReader;
#[doc = "Field `SCUDISPUIO063` writer - SCU_DIS_PU_IO063"]
pub type Scudispuio063W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO063` reader - SCU_DRV_IO063"]
pub type Scudrvio063R = crate::FieldReader;
#[doc = "Field `SCUDRVIO063` writer - SCU_DRV_IO063"]
pub type Scudrvio063W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO063` reader - SCU_EN_SMT_IO063"]
pub type Scuensmtio063R = crate::BitReader;
#[doc = "Field `SCUENSMTIO063` writer - SCU_EN_SMT_IO063"]
pub type Scuensmtio063W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO063` reader - SCU_EN_HV_IO063"]
pub type Scuenhvio063R = crate::BitReader;
#[doc = "Field `SCUENHVIO063` writer - SCU_EN_HV_IO063"]
pub type Scuenhvio063W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO062"]
    #[inline(always)]
    pub fn scudispdio062(&self) -> Scudispdio062R {
        Scudispdio062R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO062"]
    #[inline(always)]
    pub fn scudispuio062(&self) -> Scudispuio062R {
        Scudispuio062R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO062"]
    #[inline(always)]
    pub fn scudrvio062(&self) -> Scudrvio062R {
        Scudrvio062R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO062"]
    #[inline(always)]
    pub fn scuensmtio062(&self) -> Scuensmtio062R {
        Scuensmtio062R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO062"]
    #[inline(always)]
    pub fn scuenhvio062(&self) -> Scuenhvio062R {
        Scuenhvio062R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO063"]
    #[inline(always)]
    pub fn scudispdio063(&self) -> Scudispdio063R {
        Scudispdio063R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO063"]
    #[inline(always)]
    pub fn scudispuio063(&self) -> Scudispuio063R {
        Scudispuio063R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO063"]
    #[inline(always)]
    pub fn scudrvio063(&self) -> Scudrvio063R {
        Scudrvio063R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO063"]
    #[inline(always)]
    pub fn scuensmtio063(&self) -> Scuensmtio063R {
        Scuensmtio063R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO063"]
    #[inline(always)]
    pub fn scuenhvio063(&self) -> Scuenhvio063R {
        Scuenhvio063R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO062"]
    #[inline(always)]
    pub fn scudispdio062(&mut self) -> Scudispdio062W<Scu4fcSpec> {
        Scudispdio062W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO062"]
    #[inline(always)]
    pub fn scudispuio062(&mut self) -> Scudispuio062W<Scu4fcSpec> {
        Scudispuio062W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO062"]
    #[inline(always)]
    pub fn scudrvio062(&mut self) -> Scudrvio062W<Scu4fcSpec> {
        Scudrvio062W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO062"]
    #[inline(always)]
    pub fn scuensmtio062(&mut self) -> Scuensmtio062W<Scu4fcSpec> {
        Scuensmtio062W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO062"]
    #[inline(always)]
    pub fn scuenhvio062(&mut self) -> Scuenhvio062W<Scu4fcSpec> {
        Scuenhvio062W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO063"]
    #[inline(always)]
    pub fn scudispdio063(&mut self) -> Scudispdio063W<Scu4fcSpec> {
        Scudispdio063W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO063"]
    #[inline(always)]
    pub fn scudispuio063(&mut self) -> Scudispuio063W<Scu4fcSpec> {
        Scudispuio063W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO063"]
    #[inline(always)]
    pub fn scudrvio063(&mut self) -> Scudrvio063W<Scu4fcSpec> {
        Scudrvio063W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO063"]
    #[inline(always)]
    pub fn scuensmtio063(&mut self) -> Scuensmtio063W<Scu4fcSpec> {
        Scuensmtio063W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO063"]
    #[inline(always)]
    pub fn scuenhvio063(&mut self) -> Scuenhvio063W<Scu4fcSpec> {
        Scuenhvio063W::new(self, 25)
    }
}
#[doc = "IO Control \\#32\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4fcSpec;
impl crate::RegisterSpec for Scu4fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4fc::R`](R) reader structure"]
impl crate::Readable for Scu4fcSpec {}
#[doc = "`write(|w| ..)` method takes [`scu4fc::W`](W) writer structure"]
impl crate::Writable for Scu4fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4FC to value 0x0204_0204"]
impl crate::Resettable for Scu4fcSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
