#[doc = "Register `SCU4CC` reader"]
pub type R = crate::R<Scu4ccSpec>;
#[doc = "Register `SCU4CC` writer"]
pub type W = crate::W<Scu4ccSpec>;
#[doc = "Field `SCUDISPDIO038` reader - SCU_DIS_PD_IO038"]
pub type Scudispdio038R = crate::BitReader;
#[doc = "Field `SCUDISPDIO038` writer - SCU_DIS_PD_IO038"]
pub type Scudispdio038W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO038` reader - SCU_DIS_PU_IO038"]
pub type Scudispuio038R = crate::BitReader;
#[doc = "Field `SCUDISPUIO038` writer - SCU_DIS_PU_IO038"]
pub type Scudispuio038W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO038` reader - SCU_DRV_IO038"]
pub type Scudrvio038R = crate::FieldReader;
#[doc = "Field `SCUDRVIO038` writer - SCU_DRV_IO038"]
pub type Scudrvio038W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO038` reader - SCU_EN_SMT_IO038"]
pub type Scuensmtio038R = crate::BitReader;
#[doc = "Field `SCUENSMTIO038` writer - SCU_EN_SMT_IO038"]
pub type Scuensmtio038W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO038` reader - SCU_EN_HV_IO038"]
pub type Scuenhvio038R = crate::BitReader;
#[doc = "Field `SCUENHVIO038` writer - SCU_EN_HV_IO038"]
pub type Scuenhvio038W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO039` reader - SCU_DIS_PD_IO039"]
pub type Scudispdio039R = crate::BitReader;
#[doc = "Field `SCUDISPDIO039` writer - SCU_DIS_PD_IO039"]
pub type Scudispdio039W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO039` reader - SCU_DIS_PU_IO039"]
pub type Scudispuio039R = crate::BitReader;
#[doc = "Field `SCUDISPUIO039` writer - SCU_DIS_PU_IO039"]
pub type Scudispuio039W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO039` reader - SCU_DRV_IO039"]
pub type Scudrvio039R = crate::FieldReader;
#[doc = "Field `SCUDRVIO039` writer - SCU_DRV_IO039"]
pub type Scudrvio039W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO039` reader - SCU_EN_SMT_IO039"]
pub type Scuensmtio039R = crate::BitReader;
#[doc = "Field `SCUENSMTIO039` writer - SCU_EN_SMT_IO039"]
pub type Scuensmtio039W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO039` reader - SCU_EN_HV_IO039"]
pub type Scuenhvio039R = crate::BitReader;
#[doc = "Field `SCUENHVIO039` writer - SCU_EN_HV_IO039"]
pub type Scuenhvio039W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO038"]
    #[inline(always)]
    pub fn scudispdio038(&self) -> Scudispdio038R {
        Scudispdio038R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO038"]
    #[inline(always)]
    pub fn scudispuio038(&self) -> Scudispuio038R {
        Scudispuio038R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO038"]
    #[inline(always)]
    pub fn scudrvio038(&self) -> Scudrvio038R {
        Scudrvio038R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO038"]
    #[inline(always)]
    pub fn scuensmtio038(&self) -> Scuensmtio038R {
        Scuensmtio038R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO038"]
    #[inline(always)]
    pub fn scuenhvio038(&self) -> Scuenhvio038R {
        Scuenhvio038R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO039"]
    #[inline(always)]
    pub fn scudispdio039(&self) -> Scudispdio039R {
        Scudispdio039R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO039"]
    #[inline(always)]
    pub fn scudispuio039(&self) -> Scudispuio039R {
        Scudispuio039R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO039"]
    #[inline(always)]
    pub fn scudrvio039(&self) -> Scudrvio039R {
        Scudrvio039R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO039"]
    #[inline(always)]
    pub fn scuensmtio039(&self) -> Scuensmtio039R {
        Scuensmtio039R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO039"]
    #[inline(always)]
    pub fn scuenhvio039(&self) -> Scuenhvio039R {
        Scuenhvio039R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO038"]
    #[inline(always)]
    pub fn scudispdio038(&mut self) -> Scudispdio038W<Scu4ccSpec> {
        Scudispdio038W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO038"]
    #[inline(always)]
    pub fn scudispuio038(&mut self) -> Scudispuio038W<Scu4ccSpec> {
        Scudispuio038W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO038"]
    #[inline(always)]
    pub fn scudrvio038(&mut self) -> Scudrvio038W<Scu4ccSpec> {
        Scudrvio038W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO038"]
    #[inline(always)]
    pub fn scuensmtio038(&mut self) -> Scuensmtio038W<Scu4ccSpec> {
        Scuensmtio038W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO038"]
    #[inline(always)]
    pub fn scuenhvio038(&mut self) -> Scuenhvio038W<Scu4ccSpec> {
        Scuenhvio038W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO039"]
    #[inline(always)]
    pub fn scudispdio039(&mut self) -> Scudispdio039W<Scu4ccSpec> {
        Scudispdio039W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO039"]
    #[inline(always)]
    pub fn scudispuio039(&mut self) -> Scudispuio039W<Scu4ccSpec> {
        Scudispuio039W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO039"]
    #[inline(always)]
    pub fn scudrvio039(&mut self) -> Scudrvio039W<Scu4ccSpec> {
        Scudrvio039W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO039"]
    #[inline(always)]
    pub fn scuensmtio039(&mut self) -> Scuensmtio039W<Scu4ccSpec> {
        Scuensmtio039W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO039"]
    #[inline(always)]
    pub fn scuenhvio039(&mut self) -> Scuenhvio039W<Scu4ccSpec> {
        Scuenhvio039W::new(self, 25)
    }
}
#[doc = "IO Control \\#20\n\nYou can [`read`](crate::Reg::read) this register and get [`scu4cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu4cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu4ccSpec;
impl crate::RegisterSpec for Scu4ccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu4cc::R`](R) reader structure"]
impl crate::Readable for Scu4ccSpec {}
#[doc = "`write(|w| ..)` method takes [`scu4cc::W`](W) writer structure"]
impl crate::Writable for Scu4ccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU4CC to value 0x0204_0204"]
impl crate::Resettable for Scu4ccSpec {
    const RESET_VALUE: u32 = 0x0204_0204;
}
