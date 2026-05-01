#[doc = "Register `SCUC1C` reader"]
pub type R = crate::R<Scuc1cSpec>;
#[doc = "Register `SCUC1C` writer"]
pub type W = crate::W<Scuc1cSpec>;
#[doc = "Field `SCUREGSEC1380` reader - SCU_REG_SEC1_380"]
pub type Scuregsec1380R = crate::BitReader;
#[doc = "Field `SCUREGSEC1380` writer - SCU_REG_SEC1_380"]
pub type Scuregsec1380W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1384` reader - SCU_REG_SEC1_384"]
pub type Scuregsec1384R = crate::BitReader;
#[doc = "Field `SCUREGSEC1384` writer - SCU_REG_SEC1_384"]
pub type Scuregsec1384W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `SCUREGSEC1390` reader - SCU_REG_SEC1_390"]
pub type Scuregsec1390R = crate::BitReader;
#[doc = "Field `SCUREGSEC1390` writer - SCU_REG_SEC1_390"]
pub type Scuregsec1390W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1394` reader - SCU_REG_SEC1_394"]
pub type Scuregsec1394R = crate::BitReader;
#[doc = "Field `SCUREGSEC1394` writer - SCU_REG_SEC1_394"]
pub type Scuregsec1394W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC1398` reader - SCU_REG_SEC1_398"]
pub type Scuregsec1398R = crate::BitReader;
#[doc = "Field `SCUREGSEC1398` writer - SCU_REG_SEC1_398"]
pub type Scuregsec1398W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SCUREGSEC13A0` reader - SCU_REG_SEC1_3A0"]
pub type Scuregsec13a0R = crate::BitReader;
#[doc = "Field `SCUREGSEC13A0` writer - SCU_REG_SEC1_3A0"]
pub type Scuregsec13a0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC13A4` reader - SCU_REG_SEC1_3A4"]
pub type Scuregsec13a4R = crate::BitReader;
#[doc = "Field `SCUREGSEC13A4` writer - SCU_REG_SEC1_3A4"]
pub type Scuregsec13a4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUREGSEC13B0` reader - SCU_REG_SEC1_3B0"]
pub type Scuregsec13b0R = crate::BitReader;
#[doc = "Field `SCUREGSEC13B0` writer - SCU_REG_SEC1_3B0"]
pub type Scuregsec13b0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC13B4` reader - SCU_REG_SEC1_3B4"]
pub type Scuregsec13b4R = crate::BitReader;
#[doc = "Field `SCUREGSEC13B4` writer - SCU_REG_SEC1_3B4"]
pub type Scuregsec13b4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC13B8` reader - SCU_REG_SEC1_3B8"]
pub type Scuregsec13b8R = crate::BitReader;
#[doc = "Field `SCUREGSEC13B8` writer - SCU_REG_SEC1_3B8"]
pub type Scuregsec13b8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC13BC` reader - SCU_REG_SEC1_3BC"]
pub type Scuregsec13bcR = crate::BitReader;
#[doc = "Field `SCUREGSEC13BC` writer - SCU_REG_SEC1_3BC"]
pub type Scuregsec13bcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUREGSEC13C0` reader - SCU_REG_SEC1_3C0"]
pub type Scuregsec13c0R = crate::BitReader;
#[doc = "Field `SCUREGSEC13C0` writer - SCU_REG_SEC1_3C0"]
pub type Scuregsec13c0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC1_380"]
    #[inline(always)]
    pub fn scuregsec1380(&self) -> Scuregsec1380R {
        Scuregsec1380R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_384"]
    #[inline(always)]
    pub fn scuregsec1384(&self) -> Scuregsec1384R {
        Scuregsec1384R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SCU_REG_SEC1_390"]
    #[inline(always)]
    pub fn scuregsec1390(&self) -> Scuregsec1390R {
        Scuregsec1390R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCU_REG_SEC1_394"]
    #[inline(always)]
    pub fn scuregsec1394(&self) -> Scuregsec1394R {
        Scuregsec1394R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SCU_REG_SEC1_398"]
    #[inline(always)]
    pub fn scuregsec1398(&self) -> Scuregsec1398R {
        Scuregsec1398R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SCU_REG_SEC1_3A0"]
    #[inline(always)]
    pub fn scuregsec13a0(&self) -> Scuregsec13a0R {
        Scuregsec13a0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCU_REG_SEC1_3A4"]
    #[inline(always)]
    pub fn scuregsec13a4(&self) -> Scuregsec13a4R {
        Scuregsec13a4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_3B0"]
    #[inline(always)]
    pub fn scuregsec13b0(&self) -> Scuregsec13b0R {
        Scuregsec13b0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SCU_REG_SEC1_3B4"]
    #[inline(always)]
    pub fn scuregsec13b4(&self) -> Scuregsec13b4R {
        Scuregsec13b4R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - SCU_REG_SEC1_3B8"]
    #[inline(always)]
    pub fn scuregsec13b8(&self) -> Scuregsec13b8R {
        Scuregsec13b8R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SCU_REG_SEC1_3BC"]
    #[inline(always)]
    pub fn scuregsec13bc(&self) -> Scuregsec13bcR {
        Scuregsec13bcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCU_REG_SEC1_3C0"]
    #[inline(always)]
    pub fn scuregsec13c0(&self) -> Scuregsec13c0R {
        Scuregsec13c0R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC1_380"]
    #[inline(always)]
    pub fn scuregsec1380(&mut self) -> Scuregsec1380W<Scuc1cSpec> {
        Scuregsec1380W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_REG_SEC1_384"]
    #[inline(always)]
    pub fn scuregsec1384(&mut self) -> Scuregsec1384W<Scuc1cSpec> {
        Scuregsec1384W::new(self, 1)
    }
    #[doc = "Bit 4 - SCU_REG_SEC1_390"]
    #[inline(always)]
    pub fn scuregsec1390(&mut self) -> Scuregsec1390W<Scuc1cSpec> {
        Scuregsec1390W::new(self, 4)
    }
    #[doc = "Bit 5 - SCU_REG_SEC1_394"]
    #[inline(always)]
    pub fn scuregsec1394(&mut self) -> Scuregsec1394W<Scuc1cSpec> {
        Scuregsec1394W::new(self, 5)
    }
    #[doc = "Bit 6 - SCU_REG_SEC1_398"]
    #[inline(always)]
    pub fn scuregsec1398(&mut self) -> Scuregsec1398W<Scuc1cSpec> {
        Scuregsec1398W::new(self, 6)
    }
    #[doc = "Bit 8 - SCU_REG_SEC1_3A0"]
    #[inline(always)]
    pub fn scuregsec13a0(&mut self) -> Scuregsec13a0W<Scuc1cSpec> {
        Scuregsec13a0W::new(self, 8)
    }
    #[doc = "Bit 9 - SCU_REG_SEC1_3A4"]
    #[inline(always)]
    pub fn scuregsec13a4(&mut self) -> Scuregsec13a4W<Scuc1cSpec> {
        Scuregsec13a4W::new(self, 9)
    }
    #[doc = "Bit 12 - SCU_REG_SEC1_3B0"]
    #[inline(always)]
    pub fn scuregsec13b0(&mut self) -> Scuregsec13b0W<Scuc1cSpec> {
        Scuregsec13b0W::new(self, 12)
    }
    #[doc = "Bit 13 - SCU_REG_SEC1_3B4"]
    #[inline(always)]
    pub fn scuregsec13b4(&mut self) -> Scuregsec13b4W<Scuc1cSpec> {
        Scuregsec13b4W::new(self, 13)
    }
    #[doc = "Bit 14 - SCU_REG_SEC1_3B8"]
    #[inline(always)]
    pub fn scuregsec13b8(&mut self) -> Scuregsec13b8W<Scuc1cSpec> {
        Scuregsec13b8W::new(self, 14)
    }
    #[doc = "Bit 15 - SCU_REG_SEC1_3BC"]
    #[inline(always)]
    pub fn scuregsec13bc(&mut self) -> Scuregsec13bcW<Scuc1cSpec> {
        Scuregsec13bcW::new(self, 15)
    }
    #[doc = "Bit 16 - SCU_REG_SEC1_3C0"]
    #[inline(always)]
    pub fn scuregsec13c0(&mut self) -> Scuregsec13c0W<Scuc1cSpec> {
        Scuregsec13c0W::new(self, 16)
    }
}
#[doc = "Secure1 Control 8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scuc1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scuc1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scuc1cSpec;
impl crate::RegisterSpec for Scuc1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scuc1c::R`](R) reader structure"]
impl crate::Readable for Scuc1cSpec {}
#[doc = "`write(|w| ..)` method takes [`scuc1c::W`](W) writer structure"]
impl crate::Writable for Scuc1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUC1C to value 0"]
impl crate::Resettable for Scuc1cSpec {}
