#[doc = "Register `SCUD1C` reader"]
pub type R = crate::R<Scud1cSpec>;
#[doc = "Register `SCUD1C` writer"]
pub type W = crate::W<Scud1cSpec>;
#[doc = "Field `SCUREGSEC3380` reader - SCU_REG_SEC3_380"]
pub type Scuregsec3380R = crate::BitReader;
#[doc = "Field `SCUREGSEC3380` writer - SCU_REG_SEC3_380"]
pub type Scuregsec3380W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3384` reader - SCU_REG_SEC3_384"]
pub type Scuregsec3384R = crate::BitReader;
#[doc = "Field `SCUREGSEC3384` writer - SCU_REG_SEC3_384"]
pub type Scuregsec3384W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGSEC3390` reader - SCU_REG_SEC3_390"]
pub type Scuregsec3390R = crate::BitReader;
#[doc = "Field `SCUREGSEC3390` writer - SCU_REG_SEC3_390"]
pub type Scuregsec3390W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3394` reader - SCU_REG_SEC3_394"]
pub type Scuregsec3394R = crate::BitReader;
#[doc = "Field `SCUREGSEC3394` writer - SCU_REG_SEC3_394"]
pub type Scuregsec3394W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC3398` reader - SCU_REG_SEC3_398"]
pub type Scuregsec3398R = crate::BitReader;
#[doc = "Field `SCUREGSEC3398` writer - SCU_REG_SEC3_398"]
pub type Scuregsec3398W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUREGSEC33A0` reader - SCU_REG_SEC3_3A0"]
pub type Scuregsec33a0R = crate::BitReader;
#[doc = "Field `SCUREGSEC33A0` writer - SCU_REG_SEC3_3A0"]
pub type Scuregsec33a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC33A4` reader - SCU_REG_SEC3_3A4"]
pub type Scuregsec33a4R = crate::BitReader;
#[doc = "Field `SCUREGSEC33A4` writer - SCU_REG_SEC3_3A4"]
pub type Scuregsec33a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC33B0` reader - SCU_REG_SEC3_3B0"]
pub type Scuregsec33b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC33B0` writer - SCU_REG_SEC3_3B0"]
pub type Scuregsec33b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC33B4` reader - SCU_REG_SEC3_3B4"]
pub type Scuregsec33b4R = crate::BitReader;
#[doc = "Field `SCUREGSEC33B4` writer - SCU_REG_SEC3_3B4"]
pub type Scuregsec33b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC33B8` reader - SCU_REG_SEC3_3B8"]
pub type Scuregsec33b8R = crate::BitReader;
#[doc = "Field `SCUREGSEC33B8` writer - SCU_REG_SEC3_3B8"]
pub type Scuregsec33b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC33BC` reader - SCU_REG_SEC3_3BC"]
pub type Scuregsec33bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC33BC` writer - SCU_REG_SEC3_3BC"]
pub type Scuregsec33bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC33C0` reader - SCU_REG_SEC3_3C0"]
pub type Scuregsec33c0R = crate::BitReader;
#[doc = "Field `SCUREGSEC33C0` writer - SCU_REG_SEC3_3C0"]
pub type Scuregsec33c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC3_380"]
    #[inline(always)]
    pub fn scuregsec3380(&self) -> Scuregsec3380R {
        Scuregsec3380R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_384"]
    #[inline(always)]
    pub fn scuregsec3384(&self) -> Scuregsec3384R {
        Scuregsec3384R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_SEC3_390"]
    #[inline(always)]
    pub fn scuregsec3390(&self) -> Scuregsec3390R {
        Scuregsec3390R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC3_394"]
    #[inline(always)]
    pub fn scuregsec3394(&self) -> Scuregsec3394R {
        Scuregsec3394R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC3_398"]
    #[inline(always)]
    pub fn scuregsec3398(&self) -> Scuregsec3398R {
        Scuregsec3398R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_SEC3_3A0"]
    #[inline(always)]
    pub fn scuregsec33a0(&self) -> Scuregsec33a0R {
        Scuregsec33a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_SEC3_3A4"]
    #[inline(always)]
    pub fn scuregsec33a4(&self) -> Scuregsec33a4R {
        Scuregsec33a4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_3B0"]
    #[inline(always)]
    pub fn scuregsec33b0(&self) -> Scuregsec33b0R {
        Scuregsec33b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC3_3B4"]
    #[inline(always)]
    pub fn scuregsec33b4(&self) -> Scuregsec33b4R {
        Scuregsec33b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_SEC3_3B8"]
    #[inline(always)]
    pub fn scuregsec33b8(&self) -> Scuregsec33b8R {
        Scuregsec33b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_SEC3_3BC"]
    #[inline(always)]
    pub fn scuregsec33bc(&self) -> Scuregsec33bcR {
        Scuregsec33bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_SEC3_3C0"]
    #[inline(always)]
    pub fn scuregsec33c0(&self) -> Scuregsec33c0R {
        Scuregsec33c0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC3_380"]
    #[inline(always)]
    pub fn scuregsec3380(&mut self) -> Scuregsec3380W<Scud1cSpec> {
        Scuregsec3380W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC3_384"]
    #[inline(always)]
    pub fn scuregsec3384(&mut self) -> Scuregsec3384W<Scud1cSpec> {
        Scuregsec3384W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_SEC3_390"]
    #[inline(always)]
    pub fn scuregsec3390(&mut self) -> Scuregsec3390W<Scud1cSpec> {
        Scuregsec3390W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC3_394"]
    #[inline(always)]
    pub fn scuregsec3394(&mut self) -> Scuregsec3394W<Scud1cSpec> {
        Scuregsec3394W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC3_398"]
    #[inline(always)]
    pub fn scuregsec3398(&mut self) -> Scuregsec3398W<Scud1cSpec> {
        Scuregsec3398W::new(self, 6)
    }
    #[doc = "Bit 8 - SCU_REG_SEC3_3A0"]
    #[inline(always)]
    pub fn scuregsec33a0(&mut self) -> Scuregsec33a0W<Scud1cSpec> {
        Scuregsec33a0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_SEC3_3A4"]
    #[inline(always)]
    pub fn scuregsec33a4(&mut self) -> Scuregsec33a4W<Scud1cSpec> {
        Scuregsec33a4W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_SEC3_3B0"]
    #[inline(always)]
    pub fn scuregsec33b0(&mut self) -> Scuregsec33b0W<Scud1cSpec> {
        Scuregsec33b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC3_3B4"]
    #[inline(always)]
    pub fn scuregsec33b4(&mut self) -> Scuregsec33b4W<Scud1cSpec> {
        Scuregsec33b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_SEC3_3B8"]
    #[inline(always)]
    pub fn scuregsec33b8(&mut self) -> Scuregsec33b8W<Scud1cSpec> {
        Scuregsec33b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_SEC3_3BC"]
    #[inline(always)]
    pub fn scuregsec33bc(&mut self) -> Scuregsec33bcW<Scud1cSpec> {
        Scuregsec33bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_SEC3_3C0"]
    #[inline(always)]
    pub fn scuregsec33c0(&mut self) -> Scuregsec33c0W<Scud1cSpec> {
        Scuregsec33c0W::new(self, 16)
    }
}
#[doc = "Secure3 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud1cSpec;
impl crate::RegisterSpec for Scud1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud1c::R`](R) reader structure"]
impl crate::Readable for Scud1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scud1c::W`](W) writer structure"]
impl crate::Writable for Scud1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD1C to value 0"]
impl crate::Resettable for Scud1cSpec {}
