#[doc = "Register `SCU5AC` reader"]
pub type R = crate::R<Scu5acSpec>;
#[doc = "Register `SCU5AC` writer"]
pub type W = crate::W<Scu5acSpec>;
#[doc = "Field `SCUDISPDIO150` reader - SCU_DIS_PD_IO150"]
pub type Scudispdio150R = crate::BitReader;
#[doc = "Field `SCUDISPDIO150` writer - SCU_DIS_PD_IO150"]
pub type Scudispdio150W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `SCUDISPUIO150` reader - SCU_DIS_PU_IO150"]
pub type Scudispuio150R = crate::BitReader;
#[doc = "Field `SCUDISPUIO150` writer - SCU_DIS_PU_IO150"]
pub type Scudispuio150W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `SCUDRVIO150` reader - SCU_DRV_IO150"]
pub type Scudrvio150R = crate::FieldReader;
#[doc = "Field `SCUDRVIO150` writer - SCU_DRV_IO150"]
pub type Scudrvio150W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO150` reader - SCU_EN_SMT_IO150"]
pub type Scuensmtio150R = crate::BitReader;
#[doc = "Field `SCUENSMTIO150` writer - SCU_EN_SMT_IO150"]
pub type Scuensmtio150W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO150` reader - SCU_EN_HV_IO150"]
pub type Scuenhvio150R = crate::BitReader;
#[doc = "Field `SCUENHVIO150` writer - SCU_EN_HV_IO150"]
pub type Scuenhvio150W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUDISPDIO151` reader - SCU_DIS_PD_IO151"]
pub type Scudispdio151R = crate::BitReader;
#[doc = "Field `SCUDISPDIO151` writer - SCU_DIS_PD_IO151"]
pub type Scudispdio151W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUDISPUIO151` reader - SCU_DIS_PU_IO151"]
pub type Scudispuio151R = crate::BitReader;
#[doc = "Field `SCUDISPUIO151` writer - SCU_DIS_PU_IO151"]
pub type Scudispuio151W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `SCUDRVIO151` reader - SCU_DRV_IO151"]
pub type Scudrvio151R = crate::FieldReader;
#[doc = "Field `SCUDRVIO151` writer - SCU_DRV_IO151"]
pub type Scudrvio151W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCUENSMTIO151` reader - SCU_EN_SMT_IO151"]
pub type Scuensmtio151R = crate::BitReader;
#[doc = "Field `SCUENSMTIO151` writer - SCU_EN_SMT_IO151"]
pub type Scuensmtio151W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUENHVIO151` reader - SCU_EN_HV_IO151"]
pub type Scuenhvio151R = crate::BitReader;
#[doc = "Field `SCUENHVIO151` writer - SCU_EN_HV_IO151"]
pub type Scuenhvio151W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_DIS_PD_IO150"]
    #[inline(always)]
    pub fn scudispdio150(&self) -> Scudispdio150R {
        Scudispdio150R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO150"]
    #[inline(always)]
    pub fn scudispuio150(&self) -> Scudispuio150R {
        Scudispuio150R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO150"]
    #[inline(always)]
    pub fn scudrvio150(&self) -> Scudrvio150R {
        Scudrvio150R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO150"]
    #[inline(always)]
    pub fn scuensmtio150(&self) -> Scuensmtio150R {
        Scuensmtio150R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO150"]
    #[inline(always)]
    pub fn scuenhvio150(&self) -> Scuenhvio150R {
        Scuenhvio150R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:15 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 10) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO151"]
    #[inline(always)]
    pub fn scudispdio151(&self) -> Scudispdio151R {
        Scudispdio151R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO151"]
    #[inline(always)]
    pub fn scudispuio151(&self) -> Scudispuio151R {
        Scudispuio151R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO151"]
    #[inline(always)]
    pub fn scudrvio151(&self) -> Scudrvio151R {
        Scudrvio151R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO151"]
    #[inline(always)]
    pub fn scuensmtio151(&self) -> Scuensmtio151R {
        Scuensmtio151R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO151"]
    #[inline(always)]
    pub fn scuenhvio151(&self) -> Scuenhvio151R {
        Scuenhvio151R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_DIS_PD_IO150"]
    #[inline(always)]
    pub fn scudispdio150(&mut self) -> Scudispdio150W<Scu5acSpec> {
        Scudispdio150W::new(self, 0)
    }
    #[doc = "Bit 2 - SCU_DIS_PU_IO150"]
    #[inline(always)]
    pub fn scudispuio150(&mut self) -> Scudispuio150W<Scu5acSpec> {
        Scudispuio150W::new(self, 2)
    }
    #[doc = "Bits 4:7 - SCU_DRV_IO150"]
    #[inline(always)]
    pub fn scudrvio150(&mut self) -> Scudrvio150W<Scu5acSpec> {
        Scudrvio150W::new(self, 4)
    }
    #[doc = "Bit 8 - SCU_EN_SMT_IO150"]
    #[inline(always)]
    pub fn scuensmtio150(&mut self) -> Scuensmtio150W<Scu5acSpec> {
        Scuensmtio150W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_EN_HV_IO150"]
    #[inline(always)]
    pub fn scuenhvio150(&mut self) -> Scuenhvio150W<Scu5acSpec> {
        Scuenhvio150W::new(self, 9)
    }
    #[doc = "Bit 16 - SCU_DIS_PD_IO151"]
    #[inline(always)]
    pub fn scudispdio151(&mut self) -> Scudispdio151W<Scu5acSpec> {
        Scudispdio151W::new(self, 16)
    }
    #[doc = "Bit 18 - SCU_DIS_PU_IO151"]
    #[inline(always)]
    pub fn scudispuio151(&mut self) -> Scudispuio151W<Scu5acSpec> {
        Scudispuio151W::new(self, 18)
    }
    #[doc = "Bits 20:23 - SCU_DRV_IO151"]
    #[inline(always)]
    pub fn scudrvio151(&mut self) -> Scudrvio151W<Scu5acSpec> {
        Scudrvio151W::new(self, 20)
    }
    #[doc = "Bit 24 - SCU_EN_SMT_IO151"]
    #[inline(always)]
    pub fn scuensmtio151(&mut self) -> Scuensmtio151W<Scu5acSpec> {
        Scuensmtio151W::new(self, 24)
    }
    #[doc = "Bit 25 - SCU_EN_HV_IO151"]
    #[inline(always)]
    pub fn scuenhvio151(&mut self) -> Scuenhvio151W<Scu5acSpec> {
        Scuenhvio151W::new(self, 25)
    }
}
#[doc = "IO Control \\#76\n\nYou can [`read`](crate::Reg::read) this register and get [`scu5ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu5ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu5acSpec;
impl crate::RegisterSpec for Scu5acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu5ac::R`](R) reader structure"]
impl crate::Readable for Scu5acSpec {}
#[doc = "`write(|w| ..)` method takes [`scu5ac::W`](W) writer structure"]
impl crate::Writable for Scu5acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU5AC to value 0x0201_0201"]
impl crate::Resettable for Scu5acSpec {
    const RESET_VALUE: u32 = 0x0201_0201;
}
