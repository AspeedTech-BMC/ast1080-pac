#[doc = "Register `SCUC9C` reader"]
pub type R = crate::R<Scuc9cSpec>;
#[doc = "Register `SCUC9C` writer"]
pub type W = crate::W<Scuc9cSpec>;
#[doc = "Field `SCUREGSEC2380` reader - SCU_REG_SEC2_380"]
pub type Scuregsec2380R = crate::BitReader;
#[doc = "Field `SCUREGSEC2380` writer - SCU_REG_SEC2_380"]
pub type Scuregsec2380W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2384` reader - SCU_REG_SEC2_384"]
pub type Scuregsec2384R = crate::BitReader;
#[doc = "Field `SCUREGSEC2384` writer - SCU_REG_SEC2_384"]
pub type Scuregsec2384W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGSEC2390` reader - SCU_REG_SEC2_390"]
pub type Scuregsec2390R = crate::BitReader;
#[doc = "Field `SCUREGSEC2390` writer - SCU_REG_SEC2_390"]
pub type Scuregsec2390W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2394` reader - SCU_REG_SEC2_394"]
pub type Scuregsec2394R = crate::BitReader;
#[doc = "Field `SCUREGSEC2394` writer - SCU_REG_SEC2_394"]
pub type Scuregsec2394W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC2398` reader - SCU_REG_SEC2_398"]
pub type Scuregsec2398R = crate::BitReader;
#[doc = "Field `SCUREGSEC2398` writer - SCU_REG_SEC2_398"]
pub type Scuregsec2398W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUREGSEC23A0` reader - SCU_REG_SEC2_3A0"]
pub type Scuregsec23a0R = crate::BitReader;
#[doc = "Field `SCUREGSEC23A0` writer - SCU_REG_SEC2_3A0"]
pub type Scuregsec23a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC23A4` reader - SCU_REG_SEC2_3A4"]
pub type Scuregsec23a4R = crate::BitReader;
#[doc = "Field `SCUREGSEC23A4` writer - SCU_REG_SEC2_3A4"]
pub type Scuregsec23a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC23B0` reader - SCU_REG_SEC2_3B0"]
pub type Scuregsec23b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC23B0` writer - SCU_REG_SEC2_3B0"]
pub type Scuregsec23b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC23B4` reader - SCU_REG_SEC2_3B4"]
pub type Scuregsec23b4R = crate::BitReader;
#[doc = "Field `SCUREGSEC23B4` writer - SCU_REG_SEC2_3B4"]
pub type Scuregsec23b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC23B8` reader - SCU_REG_SEC2_3B8"]
pub type Scuregsec23b8R = crate::BitReader;
#[doc = "Field `SCUREGSEC23B8` writer - SCU_REG_SEC2_3B8"]
pub type Scuregsec23b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC23BC` reader - SCU_REG_SEC2_3BC"]
pub type Scuregsec23bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC23BC` writer - SCU_REG_SEC2_3BC"]
pub type Scuregsec23bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC23C0` reader - SCU_REG_SEC2_3C0"]
pub type Scuregsec23c0R = crate::BitReader;
#[doc = "Field `SCUREGSEC23C0` writer - SCU_REG_SEC2_3C0"]
pub type Scuregsec23c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC2_380"]
    #[inline(always)]
    pub fn scuregsec2380(&self) -> Scuregsec2380R {
        Scuregsec2380R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_384"]
    #[inline(always)]
    pub fn scuregsec2384(&self) -> Scuregsec2384R {
        Scuregsec2384R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_SEC2_390"]
    #[inline(always)]
    pub fn scuregsec2390(&self) -> Scuregsec2390R {
        Scuregsec2390R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC2_394"]
    #[inline(always)]
    pub fn scuregsec2394(&self) -> Scuregsec2394R {
        Scuregsec2394R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC2_398"]
    #[inline(always)]
    pub fn scuregsec2398(&self) -> Scuregsec2398R {
        Scuregsec2398R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_SEC2_3A0"]
    #[inline(always)]
    pub fn scuregsec23a0(&self) -> Scuregsec23a0R {
        Scuregsec23a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_SEC2_3A4"]
    #[inline(always)]
    pub fn scuregsec23a4(&self) -> Scuregsec23a4R {
        Scuregsec23a4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_3B0"]
    #[inline(always)]
    pub fn scuregsec23b0(&self) -> Scuregsec23b0R {
        Scuregsec23b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC2_3B4"]
    #[inline(always)]
    pub fn scuregsec23b4(&self) -> Scuregsec23b4R {
        Scuregsec23b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_SEC2_3B8"]
    #[inline(always)]
    pub fn scuregsec23b8(&self) -> Scuregsec23b8R {
        Scuregsec23b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_SEC2_3BC"]
    #[inline(always)]
    pub fn scuregsec23bc(&self) -> Scuregsec23bcR {
        Scuregsec23bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_SEC2_3C0"]
    #[inline(always)]
    pub fn scuregsec23c0(&self) -> Scuregsec23c0R {
        Scuregsec23c0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC2_380"]
    #[inline(always)]
    pub fn scuregsec2380(&mut self) -> Scuregsec2380W<Scuc9cSpec> {
        Scuregsec2380W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC2_384"]
    #[inline(always)]
    pub fn scuregsec2384(&mut self) -> Scuregsec2384W<Scuc9cSpec> {
        Scuregsec2384W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_SEC2_390"]
    #[inline(always)]
    pub fn scuregsec2390(&mut self) -> Scuregsec2390W<Scuc9cSpec> {
        Scuregsec2390W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC2_394"]
    #[inline(always)]
    pub fn scuregsec2394(&mut self) -> Scuregsec2394W<Scuc9cSpec> {
        Scuregsec2394W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC2_398"]
    #[inline(always)]
    pub fn scuregsec2398(&mut self) -> Scuregsec2398W<Scuc9cSpec> {
        Scuregsec2398W::new(self, 6)
    }
    #[doc = "Bit 8 - SCU_REG_SEC2_3A0"]
    #[inline(always)]
    pub fn scuregsec23a0(&mut self) -> Scuregsec23a0W<Scuc9cSpec> {
        Scuregsec23a0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_SEC2_3A4"]
    #[inline(always)]
    pub fn scuregsec23a4(&mut self) -> Scuregsec23a4W<Scuc9cSpec> {
        Scuregsec23a4W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_SEC2_3B0"]
    #[inline(always)]
    pub fn scuregsec23b0(&mut self) -> Scuregsec23b0W<Scuc9cSpec> {
        Scuregsec23b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC2_3B4"]
    #[inline(always)]
    pub fn scuregsec23b4(&mut self) -> Scuregsec23b4W<Scuc9cSpec> {
        Scuregsec23b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_SEC2_3B8"]
    #[inline(always)]
    pub fn scuregsec23b8(&mut self) -> Scuregsec23b8W<Scuc9cSpec> {
        Scuregsec23b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_SEC2_3BC"]
    #[inline(always)]
    pub fn scuregsec23bc(&mut self) -> Scuregsec23bcW<Scuc9cSpec> {
        Scuregsec23bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_SEC2_3C0"]
    #[inline(always)]
    pub fn scuregsec23c0(&mut self) -> Scuregsec23c0W<Scuc9cSpec> {
        Scuregsec23c0W::new(self, 16)
    }
}
#[doc = "Secure2 Control 7 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc9c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc9c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc9cSpec;
impl crate::RegisterSpec for Scuc9cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc9c::R`](R) reader structure"]
impl crate::Readable for Scuc9cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuc9c::W`](W) writer structure"]
impl crate::Writable for Scuc9cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC9C to value 0"]
impl crate::Resettable for Scuc9cSpec {}
