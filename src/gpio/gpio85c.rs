#[doc = "Register `GPIO85C` reader"]
pub type R = crate::R<Gpio85cSpec>;
#[doc = "Register `GPIO85C` writer"]
pub type W = crate::W<Gpio85cSpec>;
#[doc = "Field `GPIO076WrPrivilegeOfMaster` reader - GPIO076 Write Privilege of Master"]
pub type Gpio076wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO076WrPrivilegeOfMaster` writer - GPIO076 Write Privilege of Master"]
pub type Gpio076wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO077WrPrivilegeOfMaster` reader - GPIO077 Write Privilege of Master"]
pub type Gpio077wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO077WrPrivilegeOfMaster` writer - GPIO077 Write Privilege of Master"]
pub type Gpio077wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO078WrPrivilegeOfMaster` reader - GPIO078 Write Privilege of Master"]
pub type Gpio078wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO078WrPrivilegeOfMaster` writer - GPIO078 Write Privilege of Master"]
pub type Gpio078wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO079WrPrivilegeOfMaster` reader - GPIO079 Write Privilege of Master"]
pub type Gpio079wrPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO079WrPrivilegeOfMaster` writer - GPIO079 Write Privilege of Master"]
pub type Gpio079wrPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO076 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio076wr_privilege_of_master(&self) -> Gpio076wrPrivilegeOfMasterR {
        Gpio076wrPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO077 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio077wr_privilege_of_master(&self) -> Gpio077wrPrivilegeOfMasterR {
        Gpio077wrPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO078 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio078wr_privilege_of_master(&self) -> Gpio078wrPrivilegeOfMasterR {
        Gpio078wrPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO079 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio079wr_privilege_of_master(&self) -> Gpio079wrPrivilegeOfMasterR {
        Gpio079wrPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO076 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio076wr_privilege_of_master(&mut self) -> Gpio076wrPrivilegeOfMasterW<Gpio85cSpec> {
        Gpio076wrPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO077 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio077wr_privilege_of_master(&mut self) -> Gpio077wrPrivilegeOfMasterW<Gpio85cSpec> {
        Gpio077wrPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO078 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio078wr_privilege_of_master(&mut self) -> Gpio078wrPrivilegeOfMasterW<Gpio85cSpec> {
        Gpio078wrPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO079 Write Privilege of Master"]
    #[inline(always)]
    pub fn gpio079wr_privilege_of_master(&mut self) -> Gpio079wrPrivilegeOfMasterW<Gpio85cSpec> {
        Gpio079wrPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Write Privilege Control Register \\#19\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio85c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio85c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio85cSpec;
impl crate::RegisterSpec for Gpio85cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio85c::R`](R) reader structure"]
impl crate::Readable for Gpio85cSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio85c::W`](W) writer structure"]
impl crate::Writable for Gpio85cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO85C to value 0xffff_ffff"]
impl crate::Resettable for Gpio85cSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
