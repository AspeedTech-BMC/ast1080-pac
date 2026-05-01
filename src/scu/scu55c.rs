#[doc = "Register `SCU55C` reader"]
pub type R = crate::R<Scu55cSpec>;
#[doc = "Register `SCU55C` writer"]
pub type W = crate::W<Scu55cSpec>;
#[doc = "Field `SCUDISPDIO110` reader - SCU_DIS_PD_IO110"]
pub type Scudispdio110R = crate::BitReader;
#[doc = "Field `SCUDISPDIO110` writer - SCU_DIS_PD_IO110"]
pub type Scudispdio110W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO110` reader - SCU_DIS_PU_IO110"]
pub type Scudispuio110R = crate::BitReader;
#[doc = "Field `SCUDISPUIO110` writer - SCU_DIS_PU_IO110"]
pub type Scudispuio110W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO110` reader - SCU_DRV_IO110"]
pub type Scudrvio110R = crate::FieldReader;
#[doc = "Field `SCUDRVIO110` writer - SCU_DRV_IO110"]
pub type Scudrvio110W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO110` reader - SCU_EN_SMT_IO110"]
pub type Scuensmtio110R = crate::BitReader;
#[doc = "Field `SCUENSMTIO110` writer - SCU_EN_SMT_IO110"]
pub type Scuensmtio110W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO110` reader - SCU_EN_HV_IO110"]
pub type Scuenhvio110R = crate::BitReader;
#[doc = "Field `SCUENHVIO110` writer - SCU_EN_HV_IO110"]
pub type Scuenhvio110W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO111` reader - SCU_DIS_PD_IO111"]
pub type Scudispdio111R = crate::BitReader;
#[doc = "Field `SCUDISPDIO111` writer - SCU_DIS_PD_IO111"]
pub type Scudispdio111W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO111` reader - SCU_DIS_PU_IO111"]
pub type Scudispuio111R = crate::BitReader;
#[doc = "Field `SCUDISPUIO111` writer - SCU_DIS_PU_IO111"]
pub type Scudispuio111W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO111` reader - SCU_DRV_IO111"]
pub type Scudrvio111R = crate::FieldReader;
#[doc = "Field `SCUDRVIO111` writer - SCU_DRV_IO111"]
pub type Scudrvio111W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO111` reader - SCU_EN_SMT_IO111"]
pub type Scuensmtio111R = crate::BitReader;
#[doc = "Field `SCUENSMTIO111` writer - SCU_EN_SMT_IO111"]
pub type Scuensmtio111W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO111` reader - SCU_EN_HV_IO111"]
pub type Scuenhvio111R = crate::BitReader;
#[doc = "Field `SCUENHVIO111` writer - SCU_EN_HV_IO111"]
pub type Scuenhvio111W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO110"]
    #[inline(always)]
    pub fn scudispdio110(&self) -> Scudispdio110R {
        Scudispdio110R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO110"]
    #[inline(always)]
    pub fn scudispuio110(&self) -> Scudispuio110R {
        Scudispuio110R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO110"]
    #[inline(always)]
    pub fn scudrvio110(&self) -> Scudrvio110R {
        Scudrvio110R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO110"]
    #[inline(always)]
    pub fn scuensmtio110(&self) -> Scuensmtio110R {
        Scuensmtio110R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO110"]
    #[inline(always)]
    pub fn scuenhvio110(&self) -> Scuenhvio110R {
        Scuenhvio110R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO111"]
    #[inline(always)]
    pub fn scudispdio111(&self) -> Scudispdio111R {
        Scudispdio111R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO111"]
    #[inline(always)]
    pub fn scudispuio111(&self) -> Scudispuio111R {
        Scudispuio111R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO111"]
    #[inline(always)]
    pub fn scudrvio111(&self) -> Scudrvio111R {
        Scudrvio111R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO111"]
    #[inline(always)]
    pub fn scuensmtio111(&self) -> Scuensmtio111R {
        Scuensmtio111R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO111"]
    #[inline(always)]
    pub fn scuenhvio111(&self) -> Scuenhvio111R {
        Scuenhvio111R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO110"]
    #[inline(always)]
    pub fn scudispdio110(&mut self) -> Scudispdio110W<Scu55cSpec> {
        Scudispdio110W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO110"]
    #[inline(always)]
    pub fn scudispuio110(&mut self) -> Scudispuio110W<Scu55cSpec> {
        Scudispuio110W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO110"]
    #[inline(always)]
    pub fn scudrvio110(&mut self) -> Scudrvio110W<Scu55cSpec> {
        Scudrvio110W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO110"]
    #[inline(always)]
    pub fn scuensmtio110(&mut self) -> Scuensmtio110W<Scu55cSpec> {
        Scuensmtio110W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO110"]
    #[inline(always)]
    pub fn scuenhvio110(&mut self) -> Scuenhvio110W<Scu55cSpec> {
        Scuenhvio110W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO111"]
    #[inline(always)]
    pub fn scudispdio111(&mut self) -> Scudispdio111W<Scu55cSpec> {
        Scudispdio111W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO111"]
    #[inline(always)]
    pub fn scudispuio111(&mut self) -> Scudispuio111W<Scu55cSpec> {
        Scudispuio111W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO111"]
    #[inline(always)]
    pub fn scudrvio111(&mut self) -> Scudrvio111W<Scu55cSpec> {
        Scudrvio111W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO111"]
    #[inline(always)]
    pub fn scuensmtio111(&mut self) -> Scuensmtio111W<Scu55cSpec> {
        Scuensmtio111W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO111"]
    #[inline(always)]
    pub fn scuenhvio111(&mut self) -> Scuenhvio111W<Scu55cSpec> {
        Scuenhvio111W::new(self, 25)
    }
}
#[doc = "IO Control \\#56\n\nYou can [`read`](crate::Reg::read) this register and get [`scu55c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu55c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu55cSpec;
impl crate::RegisterSpec for Scu55cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu55c::R`](R) reader structure"]
impl crate::Readable for Scu55cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu55c::W`](W) writer structure"]
impl crate::Writable for Scu55cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU55C to value 0x0204_0204"]
impl crate::Resettable for Scu55cSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
